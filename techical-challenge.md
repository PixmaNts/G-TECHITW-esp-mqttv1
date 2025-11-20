# Technical Challenge V1

:dart: **Objective**:

Develop a communication system between an ESP32 and an other computer (computer B) using the MQTT protocol. The ESP32 should publish information to a specific MQTT topic whenever a designated GPIO pin is HIGH as INPUT.

:volcano:**Project architecture description:**

* The ESP32 and the computer are both connected to the same network
* A switch button is plugged to a designated GPIO pin of the ESP32
* An MQTT Broker is used, either one locally on the computer B, either a public one : broker.hivemq.com
* Everytime the button is pressed, the ESP32 detects it and publishes the string "pressed" to the mqtt topic : /esp32\_gpio
* The computer is listening on the /esp32\_gpio topic and prints a message every time it gets one &#x20;

:rocket:**Directions to follow, minimal development:**

* 💯 The ESP32 firmware should be developed using ESP-IDF framework in C or C++
* ✅ Make sure the code compile using CLI (idf.py)
* ❌ Arduino style is prohibited (no setup/loop pattern)
* 🦾 The code on the computer B should be dockerized in the langage of your choice (python for example) meaning that it should run inside a container.
* 🏗️ The code should be pushed to a github repository, for both sides, one on each directory
* 📝 The code should be commented, explaining each important step
* :blue\_book: A Readme file should be available on each directory
* 🧳 Several commits should be done along the way instead of only one containing everything in the history

:gorilla: **Challenge**:

* ESP32
  * Create a WiFi connection to a certain network and authenticate to it
  * Create MQTT connection and connect to the distant broker (local computer or broker.hivemq.com)
  * Handle GPIO pin as INPUT and detect it when it is HIGH
  * If the switch button is pressed, publish the string "pressed" to the topic "/esp32\_gpio"
* Computer B
  * Start broker using mosquitto if local broker, otherwise skip this line (using hive broker)
  * Subscribe to the topic "esp32\_gpio" and listens to it
  * Every time a message is received, print it

:bouquet:**Bonus**:

* Make the ESP32 listens to a particular topic. The computer will publish to it for a bidirectional communication.
* Implement an endless discussion between ESP32 and the computer using chatGPT for generating the conversation. Every time the button is pressed, it send the text to the computer and the computer answers back. The idea is to include HTTPS requests into the flow.

:detective:**Hints**:

* :military\_medal: Official ESP-IDF framework documentation : <https://docs.espressif.com/projects/esp-idf/en/latest/esp32/> and <https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/index.html>
* :avocado: Advice to build the ESP32 project using a docker image provided by espressif instead of installing the framework locally
* :link: If you didn't get it, the objective is to evaluate how you can use an ESP32 to interact with hardware, hardware events, external events, so you have the liberty on how you can design and develop the project
* :bug: If you manage to reach to bonus somehow, you should know that there seems to be a bug in the openai API request when doing from an ESP32 (even when using the official openai espressif dependency). Do not take it into account, try it anyway, the code will be evaluated.
* :key2: Also making requests to openai requires an API key. If needed for practical tests, you can easily create one by having an account and going on the developer page.
* :angel: Don't hesitate to ask questions if you are confused about a specific point or if something is not clear enough
* :hourglass: You have 4h / 4h30

:fingers\_crossed:**Good luck !**