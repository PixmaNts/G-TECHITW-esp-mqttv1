# MQTT Subscriber Client (Rust)

A Rust-based MQTT subscriber that listens for messages from the ESP32 GPIO button publisher.

## Overview

This application subscribes to the MQTT topic `/esp32_gpio` and prints messages whenever the ESP32 publishes a button press event. It can also publish messages to the `/esp32_commands` topic for bidirectional communication with the ESP32.

## Features

- MQTT client using `rumqttc` library
- Subscribes to `/esp32_gpio` topic to receive button press events
- **Publishes to `/esp32_commands` topic** to send commands to ESP32
- Prints received messages to console
- Dockerized for easy deployment

## Requirements

- Rust 1.75+ (or use Docker)
- Docker (optional, for containerized deployment)
- **OpenAI/OpenRouter API Key** (see [openAi_usage.md](openAi_usage.md) for setup instructions)

## Project Structure

```
mqtt_client/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml          # Rust dependencies
├── Dockerfile          # Docker build configuration
└── README.md          # This file
```

## Configuration

The MQTT broker and topics are configured in `src/main.rs`:

```rust
let broker = "broker.hivemq.com";      // MQTT broker address
let port = 1883;                        // MQTT port
let subscribe_topic = "/esp32_gpio";    // Topic to subscribe to (receive button presses)
let publish_topic = "/esp32_commands";  // Topic to publish to (send commands to ESP32)
```

### Using Local Broker

If using a local Mosquitto broker, change the broker address:

```rust
let broker = "localhost";  // or your broker IP address
```

## Building

### Local Build

```bash
cargo build --release
```

The binary will be in `target/release/mqtt_client`.

### Run Locally

**Subscribe only (listen for button presses):**
```bash
cargo run --release
```

**Publish a message to ESP32:**
```bash
cargo run --release -- "Hello ESP32"
```

Or run the binary directly:

```bash
# Subscribe only
./target/release/mqtt_client

# Publish message
./target/release/mqtt_client "Your message here"
```

## Docker Deployment

### Build Docker Image

```bash
docker build -t mqtt-subscriber .
```

### Run Container

```bash
docker run --rm mqtt-subscriber
```

The container will connect to the MQTT broker and start listening for messages.

## How It Works

1. **Connection**: Creates MQTT client and connects to the broker
2. **Subscription**: Subscribes to the `/esp32_gpio` topic to receive button press events
3. **Publishing** (optional): If a message is provided as command-line argument, publishes it to `/esp32_commands` topic
4. **Event Loop**: Continuously polls for incoming messages
5. **Message Handling**: Prints received messages to console

## Expected Output

### Subscribe Mode (No Arguments)

When running without arguments, the client subscribes and waits for messages:

```
Starting MQTT client...
Subscribed to topic: /esp32_gpio
No message provided. Usage: cargo run -- "<message>"
Waiting for messages from ESP32...
[CONNECTED] Connected to MQTT broker: broker.hivemq.com
[RECEIVED] Topic: '/esp32_gpio' | Message: 'pressed'
[RECEIVED] Topic: '/esp32_gpio' | Message: 'pressed'
```

### Publish Mode (With Message Argument)

When running with a message argument:

```
Starting MQTT client...
Subscribed to topic: /esp32_gpio
Publishing message to /esp32_commands: Hello ESP32
Message published successfully!
[CONNECTED] Connected to MQTT broker: broker.hivemq.com
[RECEIVED] Topic: '/esp32_gpio' | Message: 'pressed'
```

## Troubleshooting

### Connection Issues

- Verify broker address is correct
- Check network connectivity (ping the broker)
- Ensure broker is accessible from your network
- For local broker, ensure it's running: `mosquitto -v`

### No Messages Received

- Verify ESP32 is connected and publishing
- Check that both ESP32 and client are using the same broker
- Verify topic name matches: `/esp32_gpio` (for receiving) or `/esp32_commands` (for sending)
- Check ESP32 serial output for connection status

### Publishing Messages

- Use command-line argument: `cargo run -- "Your message"`
- Message will be published to `/esp32_commands` topic
- ESP32 should log the received message in its serial output
- Verify ESP32 is subscribed to `/esp32_commands` (check ESP32 logs on connection)

### Docker Issues

- Ensure Docker is running
- Check that network allows container to reach MQTT broker
- For local broker, use `--network host` flag:
  ```bash
  docker run --rm --network host mqtt-subscriber
  ```

## Local Mosquitto Broker Setup

If you want to run a local MQTT broker:

### Install Mosquitto

**Ubuntu/Debian:**
```bash
sudo apt-get install mosquitto mosquitto-clients
```

**macOS:**
```bash
brew install mosquitto
```

### Start Broker

```bash
mosquitto -v
```

The broker will run on `localhost:1883` by default.

### Test with Command Line

Subscribe to topic:
```bash
mosquitto_sub -h localhost -t /esp32_gpio
```

Publish test message:
```bash
mosquitto_pub -h localhost -t /esp32_gpio -m "test"
```

## ChatGPT API Setup

**⚠️ Required for ChatGPT integration!**

Before running the client, you must set up your API credentials. See **[openAi_usage.md](openAi_usage.md)** for detailed instructions on:
- Getting an API key (OpenRouter or OpenAI)
- Setting environment variables
- Testing your setup

**Quick start (minimum required):**
```bash
export OPENROUTER_API_KEY="sk-or-v1-your-key-here"
# Base URL and model have defaults (OpenRouter + free model)
```

**Full configuration (optional):**
```bash
export OPENROUTER_API_KEY="sk-or-v1-your-key-here"
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"  # Default
export OPENROUTER_MODEL="x-ai/grok-4.1-fast"  # Default (free model)
```

## Integration Tests

Integration tests are available to verify ChatGPT API connectivity. These tests make real API calls to verify the integration works.

### Prerequisites

- Set `OPENROUTER_API_KEY` environment variable (or `OPENAI_API_KEY`)
- Optionally set `OPENROUTER_BASE_URL` (or `OPENAI_API_BASE`) for custom endpoints
- See [openAi_usage.md](openAi_usage.md) for detailed setup instructions

### Running the Tests

```bash
# Set your OpenRouter API key
export OPENROUTER_API_KEY="sk-or-v1-xxxxx"

# Set OpenRouter base URL (optional, defaults to OpenAI)
export OPENROUTER_BASE_URL="https://openrouter.ai/api/v1"

# Run the integration tests
cargo test --test integration_test -- --nocapture
```

### What the Tests Do

- **`test_chatgpt_api_integration`**: Tests a simple ChatGPT API call with a single message
- **`test_conversation_history`**: Tests conversation history with multiple messages (simulates the endless discussion flow)

### Example Output

```
Running tests/integration_test.rs
Testing ChatGPT API integration...
Using endpoint: https://openrouter.ai/api/v1
Sending request to ChatGPT API...
✅ Success! ChatGPT response: Hello from integration test!
✅ Integration test passed!

Testing conversation history...
Using endpoint: https://openrouter.ai/api/v1
Sending conversation to ChatGPT API...
✅ Success! ChatGPT continued the story: [story continuation...]
✅ Conversation history test passed!
```

### Troubleshooting Tests

- **Missing API key**: Ensure `OPENROUTER_API_KEY` or `OPENAI_API_KEY` is set
- **Connection errors**: Check your internet connection and API endpoint URL
- **API errors**: Verify your API key is valid and has sufficient credits/quota

## Dependencies

- **rumqttc**: MQTT client library for Rust
- **tokio**: Async runtime (required by rumqttc)
- **openai-api-rs**: OpenAI API client library for Rust

See `Cargo.toml` for exact versions.

## License

This project is part of a technical challenge implementation.

