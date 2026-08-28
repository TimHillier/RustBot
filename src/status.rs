use axum::{Json, extract::State, routing::get};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct StatusState {
    inner: Arc<Mutex<StatusInner>>,
    started_at: Instant,
}

struct StatusInner {
    name: String,
    pfp: String,
    online: bool,
}

#[derive(Serialize)]
struct BotStatus {
    name: String,
    pfp: String,
    version: String,
    uptime_seconds: u64,
    online: bool,
}

impl StatusState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(StatusInner {
                name: "RustBot".to_string(),
                pfp: String::new(),
                online: false,
            })),
            started_at: Instant::now(),
        }
    }

    pub fn set_ready(&self, name: String, pfp: String) {
        let mut inner = self.inner.lock().expect("status mutex");
        inner.name = name;
        inner.pfp = pfp;
        inner.online = true;
    }

    fn snapshot(&self) -> BotStatus {
        let inner = self.inner.lock().expect("status mutex");
        BotStatus {
            name: inner.name.clone(),
            pfp: inner.pfp.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: self.started_at.elapsed().as_secs(),
            online: inner.online
        }
    }
}

pub async fn serve(state: StatusState) {
    let app = axum::Router::new()
        .route("/status", get(status))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3847")
        .await
        .expect("failed to bind to port 3847");
    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}

async fn status (
    axum::extract::State(state): axum::extract::State<StatusState>,
)-> Json<BotStatus> {
    Json(state.snapshot())
}