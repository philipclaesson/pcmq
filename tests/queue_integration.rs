use pcmq::queue::MessageQueue;
use pcmq::db;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::fs;

fn setup_test_db() -> Pool<SqliteConnectionManager> {
    // Create a temporary database file
    let db_path = "test_queue.db";
    if std::path::Path::new(db_path).exists() {
        fs::remove_file(db_path).expect("Failed to remove existing test database");
    }

    // Use the project's database initialization with custom path
    db::init_db_with_path(db_path).expect("Failed to initialize test database")
}

#[test]
fn test_queue_write_and_read() {
    // Setup
    let pool = setup_test_db();
    let queue = MessageQueue::new(pool);

    // Test data
    let test_topic = "test_topic".to_string();
    let test_body = "Hello, Queue!".to_string();

    // Write to queue
    let message_id = queue
        .enqueue(test_topic.clone(), test_body.clone())
        .expect("Failed to enqueue message");

    // Read from queue
    let message = queue
        .dequeue(test_topic.clone())
        .expect("Failed to dequeue message");

    // Verify message content
    assert_eq!(message.id, message_id.to_string());
    assert_eq!(message.topic, test_topic);
    assert_eq!(message.body, test_body);

    // Cleanup
    fs::remove_file("test_queue.db").expect("Failed to remove test database");
}