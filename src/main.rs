mod api;
mod db;
mod queue;

#[tokio::main]
async fn main() {
    let db_connection_pool = db::init_db().expect("Failed to init db");
    let queue = queue::MessageQueue::new(db_connection_pool);

    let app = api::create_router(queue);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
