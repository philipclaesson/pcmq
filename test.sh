#! /bin/zsh

ID=$(curl -X POST http://localhost:3000/enqueue -H "Content-Type: application/json" -d '{"topic": "test", "body": "hello"}' | jq -r .id)
echo "\nAdded message with id $ID"
ID2=$(curl -X POST http://localhost:3000/enqueue -H "Content-Type: application/json" -d '{"topic": "test", "body": "hello2"}' | jq -r .id)
echo "\nAdded message with id $ID2"

echo "\nDequeueing message with id $ID"
curl -X POST http://localhost:3000/dequeue -H "Content-Type: application/json" -d '{"topic": "test"}'

echo "\nStill getting message with id $ID2"
curl -X POST http://localhost:3000/dequeue -H "Content-Type: application/json" -d '{"topic": "test"}'

# Acknowledge the first message 
echo "\nAcknowledging message with id $ID"
curl -X POST http://localhost:3000/acknowledge -H "Content-Type: application/json" -d "{\"id\": \"$ID\"}"

# Dequeue the second message
echo "Dequeueing message with id $ID2"
SHOULD_BE_ID_2=$(curl -X POST http://localhost:3000/dequeue -H "Content-Type: application/json" -d '{"topic": "test"}' | jq -r .id)
if [ "$SHOULD_BE_ID_2" != "$ID2" ]; then
    echo "\nFailed to dequeue message with id $ID2"
    exit 1
fi

echo "Done ✅"
