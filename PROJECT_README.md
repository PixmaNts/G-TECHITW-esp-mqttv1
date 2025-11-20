# ESP32 MQTT GPIO Button Project

This project implements a communication system between an ESP32 and a computer using MQTT protocol. The ESP32 monitors a GPIO pin for button presses and publishes messages to an MQTT broker, which are received by a Rust-based subscriber application.

## Project Structure

```
.
├── main/                    # ESP32 firmware (ESP-IDF)
│   ├── app_main.c          # Main application code
│   ├── CMakeLists.txt      # Component build configuration
│   ├── Kconfig.projbuild   # Menuconfig options
│   └── idf_component.yml   # Component manifest
├── computerb/              # Computer B (Rust MQTT client)
│   └── mqtt_client/        # Rust MQTT subscriber
│       ├── src/
│       │   └── main.rs     # MQTT subscriber code
│       ├── Cargo.toml      # Rust dependencies
│       ├── Dockerfile      # Docker configuration
│       └── README.md       # Client documentation
├── CMakeLists.txt          # ESP-IDF project configuration
├── Cargo.toml             # Rust workspace configuration
├── README.md               # ESP32 firmware documentation
└── techical-challenge.md   # Original challenge requirements
```

## Quick Start

### ESP32 Side

1. Configure WiFi and MQTT broker:
   ```bash
   idf.py menuconfig
   ```

2. Build and flash:
   ```bash
   idf.py build
   idf.py -p /dev/ttyUSB0 flash monitor
   ```

See [README.md](README.md) for detailed ESP32 setup instructions.

### Computer B (Rust Client)

1. Build and run:
   ```bash
   cd computerb/mqtt_client
   cargo run --release
   ```

2. Or use Docker:
   ```bash
   cd computerb/mqtt_client
   docker build -t mqtt-subscriber .
   docker run --rm mqtt-subscriber
   ```

See [computerb/mqtt_client/README.md](computerb/mqtt_client/README.md) for detailed client setup instructions.

## How It Works

1. **ESP32** connects to WiFi and MQTT broker
2. **ESP32** monitors GPIO pin for button presses
3. When button is pressed (pin goes HIGH), ESP32 publishes "pressed" to `/esp32_gpio` topic
4. **Rust client** subscribes to `/esp32_gpio` topic
5. **Rust client** prints received messages to console

## Requirements Met

✅ ESP32 firmware developed using ESP-IDF (C/C++)  
✅ Code compiles using CLI (`idf.py`)  
✅ No Arduino-style code (uses ESP-IDF patterns)  
✅ Computer B code is dockerized (Rust)  
✅ Code is commented  
✅ README files in both directories  
✅ GPIO pin configurable via menuconfig  
✅ MQTT broker configurable (local or public)  

## MQTT Configuration

- **Topic**: `/esp32_gpio`
- **Message**: `"pressed"` (when button is pressed)
- **Broker Options**:
  - Public: `mqtt://broker.hivemq.com:1883`
  - Local: `mqtt://localhost:1883` (requires Mosquitto)

## Development Environment

### ESP32 Development

- **Framework**: ESP-IDF v5.5.1
- **Language**: C
- **Build System**: CMake
- **Recommended**: Use ESP-IDF Docker image

### Rust Client Development

- **Language**: Rust
- **MQTT Library**: rumqttc
- **Runtime**: Tokio
- **Containerization**: Docker

## License

This project is part of a technical challenge implementation.

