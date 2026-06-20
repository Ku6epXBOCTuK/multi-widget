#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]

use axum::{
    Json, Router,
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, patch, post},
};
use futures_util::stream::{Stream, StreamExt};
use shared::{Activity, CreateActivityRequest, UpdateActivityRequest};
use std::{
    convert::Infallible,
    sync::{Arc, nonpoison::Mutex},
    time::Duration,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::db::Db;

mod db;

struct AppState {
    activities: Arc<Mutex<Vec<Activity>>>,
    db: Db,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db = (Db::new().await).unwrap();
    let activities = db.get_all_activities().await.unwrap();

    let activities = Arc::new(Mutex::new(activities));

    let (tx, _) = broadcast::channel::<String>(16);
    let app_state = Arc::new(AppState { tx, activities, db });

    let app = Router::new()
        .route("/activities", get(full_activities_handler))
        .route("/activities", patch(update_activity_handler))
        .route("/activities", post(create_activity_handler))
        .route("/sse", get(sse_handler))
        .route("/send", post(send_message))
        .route("/health", get(health_handler))
        .with_state(app_state);

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
            Ok(msg) => Some(Ok(Event::default().data(msg))),
            Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(skipped)) => {
                eprintln!("Клиент отстал и пропустил {} сообщений", skipped);
                Some(Ok(Event::default()
                    .event("error")
                    .data("Вы пропустили часть сообщений из-за медленной сети")))
            }
        }
    });

    Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

async fn send_message(State(state): State<Arc<AppState>>, body: String) -> &'static str {
    let _ = state.tx.send(body);
    "Сообщение отправлено вещателю"
}

async fn full_activities_handler(State(state): State<Arc<AppState>>) -> Json<Vec<Activity>> {
    let activities = state.activities.lock();

    Json(activities.clone())
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
        Ok(_) => "Ok",
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
        let new_activity = Activity::create_from(body_clone);
        activities.push(new_activity);
    }

    let db = &state.db;
    match db.create_activity(body).await {
        Ok(_) => "Ok",
        Err(_) => "Error",
    }
}

async fn health_handler() -> &'static str {
    "OK"
}
