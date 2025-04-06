# pcmq
A very simple implementation of a message queue in Rust.

## Usage

```bash
cargo run
```

## API

### Add something to the queue

```bash
curl -X POST http://localhost:3000/enqueue -H "Content-Type: application/json" -d '{"topic": "test", "body": "hello"}'
```

### Get something from the queue

```bash
curl -X POST http://localhost:3000/dequeue -H "Content-Type: application/json" -d '{"topic": "test"}'
```

### Acknowledge a message (delete it from the queue)

```bash
curl -X POST http://localhost:3000/acknowledge -H "Content-Type: application/json" -d '{"id": "123e4567-e89b-12d3-a456-426614174000"}'
```
