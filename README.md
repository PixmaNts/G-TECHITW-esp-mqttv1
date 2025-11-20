# ESP32 MQTT GPIO Button Publisher

This ESP32 application monitors a GPIO pin for button presses and publishes MQTT messages when the button is pressed.

## Overview

The ESP32 connects to a WiFi network and an MQTT broker. When a button connected to a GPIO pin is pressed (pin goes HIGH), the device calls the OpenAI API directly using the esp-iot-solution library and publishes the ChatGPT response to the MQTT topic `/esp_gpt_out`.

**Endless Discussion Feature**: The ESP32 participates in an endless ChatGPT conversation loop:
- Button press → ESP32 calls OpenAI API → Publishes to `/esp_gpt_out`
- Rust client receives → Calls OpenAI API → Publishes to `/client_gpt`
- ESP32 receives → Calls OpenAI API → Publishes to `/esp_gpt_out`
- Loop continues automatically!

## Features

- WiFi connection with configurable credentials
- MQTT client connection to configurable broker
- GPIO button monitoring with edge detection
- **ChatGPT Integration** - ESP32 calls OpenAI API directly using esp-iot-solution library
- **Endless Discussion Loop** - Automatic conversation between ESP32 and computer via ChatGPT
- Configurable GPIO pin, OpenAI API key, API URL, and initial prompt via menuconfig

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

#### ChatGPT Configuration

Navigate to: **Example Configuration**

- **OpenAI API Key**: Your OpenAI/OpenRouter API key (required for ChatGPT features)
  - Get OpenRouter key from: https://openrouter.ai/keys (recommended, free models available)
  - Get OpenAI key from: https://platform.openai.com/api-keys
  - Can also be used with OpenAI-compatible services (LM Studio, etc.)
  
- **OpenAI API URL**: API endpoint URL (default: `https://openrouter.ai/api/v1/chat/completions`)
  - Default is OpenRouter (recommended for free models)
  - For OpenAI: `https://api.openai.com/v1/chat/completions`
  - For LM Studio: `http://localhost:1234/v1/chat/completions`
  - For other services: adjust URL accordingly
  
- **OpenAI Model Name**: AI model to use (default: `x-ai/grok-4.1-fast`)
  - Default is free OpenRouter model (no credits required)
  - For OpenAI: `gpt-3.5-turbo` or `gpt-4`
  - For OpenRouter: See https://openrouter.ai/models for available models
  - Maximum 100 characters to prevent RAM overflow
  
- **Initial ChatGPT Prompt**: Initial prompt sent when button is pressed (default: "write me a story")
  - Maximum 200 characters to prevent RAM overflow
  - This starts the endless discussion loop

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

### Basic Flow

1. **Initialization**: The application initializes NVS, network interface, event loop, and OpenAI API client
2. **WiFi Connection**: Connects to the configured WiFi network
3. **MQTT Connection**: Connects to the configured MQTT broker
4. **MQTT Subscriptions**: 
   - Subscribes to `/esp32_commands` topic (for backward compatibility)
   - Subscribes to `/client_gpt` topic (to receive ChatGPT responses from Rust client)
5. **GPIO Setup**: Configures the specified GPIO pin as input with pull-down resistor
6. **Monitoring Task**: A FreeRTOS task continuously polls the GPIO pin every 50ms

### Endless Discussion Flow (ChatGPT Integration)

1. **Button Press**: When button is pressed (pin goes HIGH):
   - ESP32 calls OpenAI API with initial prompt (from menuconfig)
   - Publishes ChatGPT response to `/esp_gpt_out` topic
   
2. **Rust Client Receives**: 
   - Subscribes to `/esp_gpt_out` topic
   - Receives ChatGPT response from ESP32
   - Calls OpenAI API with conversation history
   - Publishes new ChatGPT response to `/client_gpt` topic
   
3. **ESP32 Continues**:
   - Receives message from `/client_gpt` topic
   - Calls OpenAI API with received message
   - Publishes ChatGPT response to `/esp_gpt_out` topic
   
4. **Loop Continues**: Steps 2-3 repeat automatically, creating an endless discussion!

## Code Structure

### Main Components

- **`app_main()`**: Entry point, initializes all subsystems
- **`gpio_init()`**: Configures GPIO pin as input
- **`gpio_task()`**: Background task that monitors button and publishes MQTT messages
- **`mqtt_app_start()`**: Initializes and starts MQTT client
- **`mqtt_event_handler()`**: Handles MQTT events (connection, disconnection, data reception, etc.)

### MQTT Topics

- **`/esp_gpt_out`** (Publish): ESP32 publishes ChatGPT responses to this topic
- **`/client_gpt`** (Subscribe): ESP32 receives ChatGPT responses from Rust client
- **`/esp32_gpio`** (Publish): ESP32 publishes "pressed" for backward compatibility/logging
- **`/esp32_commands`** (Subscribe): ESP32 receives commands from the computer (backward compatibility)

### Key Features

- **Edge Detection**: Only triggers on rising edge (LOW → HIGH) to avoid multiple triggers
- **Debouncing**: 50ms polling interval provides basic debouncing
- **Error Handling**: Checks if MQTT client is ready before publishing

## Expected Output

### Startup

On startup, you should see:

```
I (xxxxx) mqtt_example: OpenAI API initialized successfully
I (xxxxx) mqtt_example: MQTT_EVENT_CONNECTED
I (xxxxx) mqtt_example: Ready to publish button presses to /esp32_gpio
I (xxxxx) mqtt_example: Subscribed to /esp32_commands topic, msg_id=1
I (xxxxx) mqtt_example: Subscribed to /client_gpt topic, msg_id=2
I (xxxxx) mqtt_example: ChatGPT integration ready. Press button to start endless discussion!
```

### Button Press (Starts Endless Discussion)

When the button is pressed, you should see:

```
I (xxxxx) mqtt_example: Button pressed! Calling OpenAI API with initial prompt...
I (xxxxx) mqtt_example: Sending prompt to OpenAI: write me a story
I (xxxxx) mqtt_example: Published ChatGPT response to /esp_gpt_out, msg_id=12345
I (xxxxx) mqtt_example: Response: [ChatGPT's story response...]
```

### Receiving from Rust Client (Continuing Discussion)

When ESP32 receives a ChatGPT response from Rust client:

```
I (xxxxx) mqtt_example: MQTT_EVENT_DATA
I (xxxxx) mqtt_example: Topic: /client_gpt
I (xxxxx) mqtt_example: Data: [Rust client's ChatGPT response...]
I (xxxxx) mqtt_example: Received ChatGPT response from Rust client: [response...]
I (xxxxx) mqtt_example: Published ChatGPT response to /esp_gpt_out, msg_id=12346
I (xxxxx) mqtt_example: Response: [ESP32's ChatGPT response...]
```

The loop continues automatically!

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

### ChatGPT/OpenAI Issues

- Verify OpenAI API key is set correctly in menuconfig
- Check API URL is correct (for LM Studio or other services)
- Ensure WiFi connection is stable (required for HTTPS requests)
- Check serial output for OpenAI API error messages
- Verify esp-iot-solution component is properly included
- Note: There's a known bug in OpenAI API requests from ESP32 (mentioned in challenge hints), but code should still work

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
