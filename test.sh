#! /bin/zsh

curl -X POST http://localhost:3000/enqueue -H "Content-Type: application/json" -d '{"topic": "test", "body": "hello"}'
curl -X POST http://localhost:3000/dequeue -H "Content-Type: application/json" -d '{"topic": "test"}'

