# MQTT Subscriber Client (Rust)

A Rust-based MQTT subscriber that listens for messages from the ESP32 GPIO button publisher.

## Overview

This application subscribes to the MQTT topic `/esp32_gpio` and prints messages whenever the ESP32 publishes a button press event.

## Features

- MQTT client using `rumqttc` library
- Subscribes to `/esp32_gpio` topic
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

The MQTT broker and topic are configured in `src/main.rs`:

```rust
let broker = "broker.hivemq.com";  // MQTT broker address
let port = 1883;                    // MQTT port
let topic = "/esp32_gpio";           // Topic to subscribe to
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

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/mqtt_client
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
2. **Subscription**: Subscribes to the `esp32_gpio` topic
3. **Event Loop**: Continuously polls for incoming messages
4. **Message Handling**: Prints received messages to console

## Expected Output

When the ESP32 button is pressed, you should see:

```
Starting MQTT subscriber...
Subscribed to topic: /esp32_gpio
Waiting for messages from ESP32...
[CONNECTED] Connected to MQTT broker: broker.hivemq.com
[RECEIVED] Topic: '/esp32_gpio' | Message: 'pressed'
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
- Verify topic name matches: `/esp32_gpio`
- Check ESP32 serial output for connection status

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

