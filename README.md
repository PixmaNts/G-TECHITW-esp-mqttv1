# ESP32 MQTT GPIO Button Publisher

This ESP32 application monitors a GPIO pin for button presses and publishes MQTT messages when the button is pressed.

## Overview

The ESP32 connects to a WiFi network and an MQTT broker. When a button connected to a GPIO pin is pressed (pin goes HIGH), the device publishes the string "pressed" to the MQTT topic `/esp32_gpio`.

**Bidirectional Communication**: The ESP32 also subscribes to the `/esp32_commands` topic and can receive messages from the computer, enabling two-way communication.

## Features

- WiFi connection with configurable credentials
- MQTT client connection to configurable broker
- GPIO button monitoring with edge detection
- Automatic MQTT message publishing on button press
- **Bidirectional MQTT communication** - ESP32 can receive commands from computer
- Configurable GPIO pin via menuconfig

## Hardware Requirements

- ESP32 development board
- Button/switch connected to a GPIO pin
  - One side connected to the GPIO pin
  - Other side connected to 3.3V (when pressed, pin goes HIGH)
  - Internal pull-down resistor is enabled

## Software Requirements

- ESP-IDF v5.5.1 or compatible version
- Python 3.x (for ESP-IDF tools)

## Project Structure

```
.
├── main/
│   ├── app_main.c          # Main application code
│   ├── CMakeLists.txt      # Component build configuration
│   ├── Kconfig.projbuild   # Menuconfig options
│   └── idf_component.yml   # Component manifest
├── CMakeLists.txt          # Project build configuration
├── sdkconfig              # Build configuration (generated)
└── README.md              # This file
```

## Configuration

### 1. Set Target

```bash
idf.py set-target esp32
```

### 2. Configure Project

Open the configuration menu:

```bash
idf.py menuconfig
```

#### WiFi Configuration

Navigate to: **Example Connection Configuration**

- Set **WiFi SSID**: Your WiFi network name
- Set **WiFi Password**: Your WiFi password

#### MQTT Configuration

Navigate to: **Example Configuration**

- **Broker URL**: MQTT broker address
  - For public broker: `mqtt://broker.hivemq.com:1883`
  - For local broker: `mqtt://192.168.1.100:1883` (replace with your broker IP)

#### GPIO Configuration

Navigate to: **Example Configuration**

- **GPIO Button Pin**: GPIO pin number for the button (default: 4)
  - Valid range: 0-39 for ESP32
  - Avoid GPIO 6-11 (used for flash)

Save configuration and exit (press `S` then `Q`).

## Building

Build the project:

```bash
idf.py build
```

## Flashing and Monitoring

Flash the firmware to the ESP32 and monitor serial output:

```bash
idf.py -p /dev/ttyUSB0 flash monitor
```

Replace `/dev/ttyUSB0` with your serial port (on Windows it might be `COM3`, on macOS `/dev/cu.usbserial-*`).

To exit the monitor, press `Ctrl+]`.

## How It Works

1. **Initialization**: The application initializes NVS, network interface, and event loop
2. **WiFi Connection**: Connects to the configured WiFi network
3. **MQTT Connection**: Connects to the configured MQTT broker
4. **MQTT Subscription**: Automatically subscribes to `/esp32_commands` topic for receiving commands
5. **GPIO Setup**: Configures the specified GPIO pin as input with pull-down resistor
6. **Monitoring Task**: A FreeRTOS task continuously polls the GPIO pin every 50ms
7. **Button Detection**: When the pin transitions from LOW to HIGH (button pressed), it publishes "pressed" to `/esp32_gpio` topic
8. **Command Reception**: When a message is received on `/esp32_commands`, it logs the topic and payload

## Code Structure

### Main Components

- **`app_main()`**: Entry point, initializes all subsystems
- **`gpio_init()`**: Configures GPIO pin as input
- **`gpio_task()`**: Background task that monitors button and publishes MQTT messages
- **`mqtt_app_start()`**: Initializes and starts MQTT client
- **`mqtt_event_handler()`**: Handles MQTT events (connection, disconnection, data reception, etc.)

### MQTT Topics

- **`/esp32_gpio`** (Publish): ESP32 publishes "pressed" when button is pressed
- **`/esp32_commands`** (Subscribe): ESP32 receives commands from the computer

### Key Features

- **Edge Detection**: Only triggers on rising edge (LOW → HIGH) to avoid multiple triggers
- **Debouncing**: 50ms polling interval provides basic debouncing
- **Error Handling**: Checks if MQTT client is ready before publishing

## Expected Output

### Button Press (ESP32 → Computer)

When the button is pressed, you should see:

```
I (xxxxx) mqtt_example: Button pressed! Published to /esp32_gpio, msg_id=12345
I (xxxxx) mqtt_example: MQTT_EVENT_PUBLISHED, msg_id=12345
```

### Command Reception (Computer → ESP32)

When the computer publishes a message to `/esp32_commands`, you should see:

```
I (xxxxx) mqtt_example: MQTT_EVENT_DATA
I (xxxxx) mqtt_example: Topic: /esp32_commands
I (xxxxx) mqtt_example: Data: Hello ESP32
```

### Connection and Subscription

On startup, you should see:

```
I (xxxxx) mqtt_example: MQTT_EVENT_CONNECTED
I (xxxxx) mqtt_example: Ready to publish button presses to /esp32_gpio
I (xxxxx) mqtt_example: Subscribed to /esp32_commands topic, msg_id=1
```

## Troubleshooting

### WiFi Connection Issues

- Verify SSID and password in menuconfig
- Check that ESP32 is in range of the access point
- Ensure WiFi network is 2.4GHz (ESP32 doesn't support 5GHz)

### MQTT Connection Issues

- Verify broker URL is correct
- Check network connectivity (ping the broker)
- Ensure broker is accessible from your network
- For local broker, check firewall settings

### Button Not Detected

- Verify GPIO pin number in menuconfig
- Check button wiring (one side to GPIO, other to 3.3V)
- Test with multimeter to verify pin goes HIGH when pressed
- Try a different GPIO pin (avoid 6-11)

## Development Setup with Docker

You can use the ESP-IDF Docker image for development:

```bash
docker run --rm -it \
  -v "$(pwd):/workspace" \
  -w /workspace \
  espressif/idf:latest \
  /bin/bash

# Inside container
source /opt/esp/idf/export.sh
idf.py build
```

## License

This example code is in the Public Domain (CC0 licensed).
