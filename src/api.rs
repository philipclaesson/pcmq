use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::queue::{Message, MessageQueue};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};


pub fn create_router(queue: MessageQueue) -> Router {
    let queue = Arc::new(queue);
    Router::new()
    // .route("/hello", get(hello))
    .route("/enqueue", post(enqueue))
    .route("/dequeue", post(dequeue))
    .route("/acknowledge", post(acknowledge))
    .with_state(queue)
}

async fn hello() -> Json<&'static str> {
    Json("hello")
}

#[derive(Deserialize)]
struct EnqueueRequest {
    body: String,
    topic: String,
}

#[derive(Serialize)]
struct EnqueueResponse {
    id: String,
}

async fn enqueue(
    State(queue): State<Arc<MessageQueue>>,
    Json(payload): Json<EnqueueRequest>,
) -> Json<EnqueueResponse> {
    let id = queue.enqueue(payload.topic, payload.body).unwrap();
    Json(EnqueueResponse { id: id.to_string() })
}

#[derive(Deserialize)]
struct DequeueRequest {
    topic: String,
}

#[derive(Serialize)]
struct DequeueResponse {
    id: String,
    body: String,
}

async fn dequeue(
    State(queue ): State<Arc<MessageQueue>>,
    Json(payload): Json<DequeueRequest>,
) -> Json<DequeueResponse> {
    let msg = queue.dequeue(payload.topic).unwrap();
    Json(DequeueResponse { id: msg.id, body: msg.body })
}

#[derive(Deserialize)]
struct AcknowledgeRequest {
    id: String,
}


async fn acknowledge(
    State(queue): State<Arc<MessageQueue>>,
    Json(payload): Json<AcknowledgeRequest>,
) -> Json<&'static str> {
    queue.acknowledge(payload.id).unwrap();
    Json("ok")
}
