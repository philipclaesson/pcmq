mod api;
mod db;

#[tokio::main]
async fn main() {
    let _db = db::init_db().expect("Failed to init db");

    let app = api::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
