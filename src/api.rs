use axum::{
    routing::get,
    Json, Router,
};


pub fn create_router() -> Router {
    Router::new().route("/hello", get(hello))
}

async fn hello() -> Json<&'static str> {
    Json("hello")
}
