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

## Dependencies

- **rumqttc**: MQTT client library for Rust
- **tokio**: Async runtime (required by rumqttc)

See `Cargo.toml` for exact versions.

## License

This project is part of a technical challenge implementation.

