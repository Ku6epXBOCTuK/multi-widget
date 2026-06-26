#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]

use axum::{
    Json, Router,
    extract::{Query, State},
    response::sse::{Event, Sse},
    routing::{get, patch, post},
};
use futures_util::stream::{Stream, StreamExt};
use shared::{
    Activity, ActivityId, CreateActivityRequest, GetActivityRequest, StreamEvent,
    UpdateActivityRequest,
};
use std::{
    convert::Infallible,
    sync::{Arc, nonpoison::Mutex},
    time::Duration,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::{Any, CorsLayer};

use crate::db::Db;

mod db;
mod types;

struct AppState {
    activities: Arc<Mutex<Vec<Activity>>>,
    db: Db,
    tx: broadcast::Sender<StreamEvent>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new().allow_origin(Any);

    let db = (Db::new().await).unwrap();
    let activities = db.get_all_activities().await.unwrap();

    let activities = Arc::new(Mutex::new(activities));

    let (tx, _) = broadcast::channel::<StreamEvent>(16);
    let app_state = Arc::new(AppState { tx, activities, db });

    let app = Router::new()
        .route("/activities", get(full_activities_handler))
        .route("/activity", get(single_activity_handler))
        .route("/activity", patch(update_activity_handler))
        .route("/activity", post(create_activity_handler))
        .route("/sse", get(sse_handler))
        .route("/health", get(health_handler))
        .with_state(app_state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();

    let stream = BroadcastStream::new(rx);

    let sse_stream = stream.filter_map(|res| async move {
        match res {
            Ok(msg) => Some(Ok(
                Event::default().data(serde_json::to_string(&msg).unwrap_or_default())
            )),
            Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(_)) => {
                // TODO: Handle client lagged
                Some(Ok(Event::default()
                    .event("error")
                    .data("You to slow for my server")))
            }
        }
    });

    Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

async fn full_activities_handler(State(state): State<Arc<AppState>>) -> Json<Vec<Activity>> {
    let activities = state.activities.lock();

    Json(activities.clone())
}

async fn single_activity_handler(
    State(state): State<Arc<AppState>>,
    query: Query<GetActivityRequest>,
) -> Json<Option<Activity>> {
    let activities = state.activities.lock();

    let activity = activities
        .iter()
        .find(|activity| activity.id == query.id)
        .cloned();

    Json(activity)
}

async fn update_activity_handler(State(state): State<Arc<AppState>>, body: String) -> &'static str {
    let body = serde_json::from_str::<UpdateActivityRequest>(&body);
    if let Err(_) = body {
        return "Error";
    }
    let body = body.unwrap();
    let body_clone = body.clone();

    {
        let mut activities = state.activities.lock();
        let body_id = body.id;
        let try_find = activities
            .iter_mut()
            .find(|activity| activity.id == body_id);
        if let Some(activity) = try_find {
            activity.update_from(body_clone);
        }
    }

    let db = &state.db;
    match db.update_activity(body).await {
        Ok(activity) => {
            let _ = state.tx.send(StreamEvent::UpdateActivity(activity));
            "Ok"
        }
        Err(_) => "Error",
    }
}

async fn create_activity_handler(State(state): State<Arc<AppState>>, body: String) -> &'static str {
    let body = serde_json::from_str::<CreateActivityRequest>(&body);
    if let Err(_) = body {
        return "Error";
    }
    let body = body.unwrap();
    let body_clone = body.clone();

    {
        let mut activities = state.activities.lock();
        let new_id = activities.iter().map(|act| act.id.0).max().unwrap_or(0) + 1;
        let new_activity = Activity::create_from(body_clone, ActivityId(new_id));
        activities.push(new_activity);
    }

    let db = &state.db;
    match db.create_activity(body).await {
        Ok(activity) => {
            let _ = state.tx.send(StreamEvent::CreateActivity(activity));
            "Ok"
        }
        Err(_) => "Error",
    }
}

async fn health_handler() -> &'static str {
    "Ok"
}
