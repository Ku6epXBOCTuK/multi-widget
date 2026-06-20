#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]

use axum::{
    Router,
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
};
use futures_util::stream::{Stream, StreamExt};
use shared::Activity;
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
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let db = (Db::new().await).unwrap();
    let activities = db.get_activities().await.unwrap();
    println!("Activities: {:?}", activities);

    let activities = Arc::new(Mutex::new(activities));

    let (tx, _) = broadcast::channel::<String>(16);
    let app_state = Arc::new(AppState { tx, activities });

    let app = Router::new()
        // .route("/activities", get(full_activities_handler))
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

async fn health_handler() -> &'static str {
    "OK"
}
