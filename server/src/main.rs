use axum::{
    Router,
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
};
use futures_util::stream::{Stream, StreamExt};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(16);
    let app_state = Arc::new(AppState { tx });

    let app = Router::new()
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
