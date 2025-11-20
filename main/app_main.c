#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <inttypes.h>
#include "esp_system.h"
#include "nvs_flash.h"
#include "esp_event.h"
#include "esp_netif.h"
#include "protocol_examples_common.h"

#include "esp_log.h"
#include "mqtt_client.h"

// GPIO includes for button reading
#include "driver/gpio.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

// OpenAI includes
#include "OpenAI.h"

static const char *TAG = "mqtt_example";

// GPIO pin number from menuconfig
#define GPIO_BUTTON_PIN CONFIG_GPIO_BUTTON_PIN

// Global MQTT client handle - needed so GPIO task can publish messages
static esp_mqtt_client_handle_t mqtt_client_handle = NULL;

// Global OpenAI handle
static OpenAI_t *openai_handle = NULL;

// Buffer for storing ChatGPT responses (limited to prevent RAM overflow)
#define MAX_RESPONSE_LEN 500
static char chatgpt_response_buffer[MAX_RESPONSE_LEN + 1];


static void log_error_if_nonzero(const char *message, int error_code)
{
    if (error_code != 0) {
        ESP_LOGE(TAG, "Last error %s: 0x%x", message, error_code);
    }
}

/*
 * @brief Event handler registered to receive MQTT events
 *
 *  This function is called by the MQTT client event loop.
 *
 * @param handler_args user data registered to the event.
 * @param base Event base for the handler(always MQTT Base in this example).
 * @param event_id The id for the received event.
 * @param event_data The data for the event, esp_mqtt_event_handle_t.
 */
/*
 * @brief Event handler registered to receive MQTT events
 */
static void mqtt_event_handler(void *handler_args, esp_event_base_t base, int32_t event_id, void *event_data)
{
    esp_mqtt_event_handle_t event = event_data;
    
    switch ((esp_mqtt_event_id_t)event_id) {
    case MQTT_EVENT_CONNECTED:
        ESP_LOGI(TAG, "MQTT_EVENT_CONNECTED");
        ESP_LOGI(TAG, "Ready to publish button presses to /esp32_gpio");
        // Subscribe to command topic for bidirectional communication
        int msg_id_sub = esp_mqtt_client_subscribe(event->client, "/esp32_commands", 0);
        ESP_LOGI(TAG, "Subscribed to /esp32_commands topic, msg_id=%d", msg_id_sub);
        // Subscribe to /client_gpt topic to receive ChatGPT responses from Rust client
        int msg_id_gpt = esp_mqtt_client_subscribe(event->client, "/client_gpt", 0);
        ESP_LOGI(TAG, "Subscribed to /client_gpt topic, msg_id=%d", msg_id_gpt);
        break;
    case MQTT_EVENT_DISCONNECTED:
        ESP_LOGI(TAG, "MQTT_EVENT_DISCONNECTED");
        break;
    case MQTT_EVENT_PUBLISHED:
        ESP_LOGI(TAG, "MQTT_EVENT_PUBLISHED, msg_id=%d", event->msg_id);
        break;
    case MQTT_EVENT_DATA:
        ESP_LOGI(TAG, "MQTT_EVENT_DATA");
        ESP_LOGI(TAG, "Topic: %.*s", event->topic_len, event->topic);
        ESP_LOGI(TAG, "Data: %.*s", event->data_len, event->data);
        
        // Check if this is a message from /client_gpt topic (ChatGPT response from Rust client)
        if (event->topic_len == 11 && strncmp(event->topic, "/client_gpt", 11) == 0) {
            // Extract the message (truncate if too long to prevent RAM overflow)
            int data_len = event->data_len;
            if (data_len > MAX_RESPONSE_LEN) {
                data_len = MAX_RESPONSE_LEN;
                ESP_LOGW(TAG, "Message truncated from %d to %d bytes", event->data_len, MAX_RESPONSE_LEN);
            }
            
            // Copy message to buffer (null-terminated)
            memcpy(chatgpt_response_buffer, event->data, data_len);
            chatgpt_response_buffer[data_len] = '\0';
            
            ESP_LOGI(TAG, "Received ChatGPT response from Rust client: %.*s", data_len, chatgpt_response_buffer);
            
            // Call OpenAI API with the received message to continue the conversation
            if (openai_handle != NULL) {
                // Create a chat completion object
                OpenAI_ChatCompletion_t *chat = openai_handle->chatCreate(openai_handle);
                if (chat != NULL) {
                    // Use model from menuconfig (defaults to free OpenRouter model)
                    chat->setModel(chat, CONFIG_OPENAI_MODEL);
                    chat->setTemperature(chat, 0.7);
                    
                    // Send the received message to OpenAI API (save=true to maintain conversation)
                    OpenAI_StringResponse_t *response = chat->message(chat, chatgpt_response_buffer, true);
                    if (response != NULL && response->getError(response) == NULL) {
                        // Get the response text
                        uint32_t len = response->getLen(response);
                        if (len > 0) {
                            char *response_text = response->getData(response, 0);
                            if (response_text != NULL) {
                                // Truncate if too long
                                int pub_len = strlen(response_text);
                                if (pub_len > MAX_RESPONSE_LEN) {
                                    pub_len = MAX_RESPONSE_LEN;
                                    ESP_LOGW(TAG, "Response truncated before publishing");
                                }
                                
                                // Publish ChatGPT response to /esp_gpt_out topic
                                int msg_id = esp_mqtt_client_publish(
                                    mqtt_client_handle,
                                    "/esp_gpt_out",
                                    response_text,
                                    pub_len,
                                    0,  // QoS 0
                                    0   // Don't retain
                                );
                                ESP_LOGI(TAG, "Published ChatGPT response to /esp_gpt_out, msg_id=%d", msg_id);
                                ESP_LOGI(TAG, "Response: %.*s", pub_len, response_text);
                            }
                        }
                        response->deleteResponse(response);
                    } else {
                        const char *error = response ? response->getError(response) : "Unknown error";
                        ESP_LOGE(TAG, "OpenAI API error: %s", error ? error : "Failed to get response");
                        if (response) {
                            response->deleteResponse(response);
                        }
                    }
                    // Clean up chat completion object
                    openai_handle->chatDelete(chat);
                } else {
                    ESP_LOGE(TAG, "Failed to create ChatCompletion object");
                }
            } else {
                ESP_LOGE(TAG, "OpenAI handle not initialized");
            }
        }
        break;
    case MQTT_EVENT_ERROR:
        ESP_LOGI(TAG, "MQTT_EVENT_ERROR");
        if (event->error_handle->error_type == MQTT_ERROR_TYPE_TCP_TRANSPORT) {
            log_error_if_nonzero("reported from esp-tls", event->error_handle->esp_tls_last_esp_err);
            log_error_if_nonzero("reported from tls stack", event->error_handle->esp_tls_stack_err);
            log_error_if_nonzero("captured as transport's socket errno",  event->error_handle->esp_transport_sock_errno);
            ESP_LOGI(TAG, "Last errno string (%s)", strerror(event->error_handle->esp_transport_sock_errno));
        }
        break;
    default:
        break;
    }

}

/*
* @brief GPIO monitoring task
* 
* This task runs in the background and continuously checks the GPIO pin.
* When the button is pressed (pin goes HIGH), it calls OpenAI API with initial prompt
* and publishes the response to /esp_gpt_out topic to start the endless discussion.
*/
static void gpio_task(void* arg)
{
    bool last_state = false;  // Track previous button state to detect edges
    
    ESP_LOGI(TAG, "GPIO monitoring task started on pin %d", GPIO_BUTTON_PIN);
    
    while (1) {
        // Read the current GPIO pin level
        int level = gpio_get_level(GPIO_BUTTON_PIN);
        
        // Detect rising edge: button was just pressed (went from LOW to HIGH)
        if (level == 1 && !last_state) {
            // Button is pressed! Call OpenAI API with initial prompt
            ESP_LOGI(TAG, "Button pressed! Calling OpenAI API with initial prompt...");
            
            if (openai_handle != NULL && mqtt_client_handle != NULL) {
                // Create a chat completion object
                OpenAI_ChatCompletion_t *chat = openai_handle->chatCreate(openai_handle);
                if (chat != NULL) {
                    // Configure chat completion - use model from menuconfig
                    chat->setModel(chat, CONFIG_OPENAI_MODEL);
                    chat->setTemperature(chat, 0.7);
                    
                    // Get initial prompt from menuconfig
                    const char *initial_prompt = CONFIG_INITIAL_PROMPT;
                    ESP_LOGI(TAG, "Sending prompt to OpenAI: %s", initial_prompt);
                    
                    // Send to OpenAI API (save=true to maintain conversation for future calls)
                    OpenAI_StringResponse_t *response = chat->message(chat, initial_prompt, true);
                    if (response != NULL && response->getError(response) == NULL) {
                        // Get the response text
                        uint32_t len = response->getLen(response);
                        if (len > 0) {
                            char *response_text = response->getData(response, 0);
                            if (response_text != NULL) {
                                // Truncate if too long to prevent RAM overflow
                                int pub_len = strlen(response_text);
                                if (pub_len > MAX_RESPONSE_LEN) {
                                    pub_len = MAX_RESPONSE_LEN;
                                    ESP_LOGW(TAG, "Response truncated before publishing");
                                }
                                
                                // Publish ChatGPT response to /esp_gpt_out topic
                                int msg_id = esp_mqtt_client_publish(
                                    mqtt_client_handle,
                                    "/esp_gpt_out",
                                    response_text,
                                    pub_len,
                                    0,  // QoS 0
                                    0   // Don't retain
                                );
                                ESP_LOGI(TAG, "Published ChatGPT response to /esp_gpt_out, msg_id=%d", msg_id);
                                ESP_LOGI(TAG, "Response: %.*s", pub_len, response_text);
                                
                                // Also publish to /esp32_gpio for backward compatibility/logging
                                esp_mqtt_client_publish(
                                    mqtt_client_handle,
                                    "/esp32_gpio",
                                    "pressed",
                                    0,
                                    0,
                                    0
                                );
                            }
                        }
                        response->deleteResponse(response);
                    } else {
                        const char *error = response ? response->getError(response) : "Unknown error";
                        ESP_LOGE(TAG, "OpenAI API error: %s", error ? error : "Failed to get response");
                        if (response) {
                            response->deleteResponse(response);
                        }
                    }
                    // Clean up chat completion object
                    openai_handle->chatDelete(chat);
                } else {
                    ESP_LOGE(TAG, "Failed to create ChatCompletion object");
                }
            } else {
                if (openai_handle == NULL) {
                    ESP_LOGW(TAG, "OpenAI handle not initialized, button press ignored");
                }
                if (mqtt_client_handle == NULL) {
                    ESP_LOGW(TAG, "MQTT client not ready yet, button press ignored");
                }
            }
        }
        
        // Update last state for next iteration
        last_state = (level == 1);
        
        // Wait 50ms before checking again (debouncing + reduces CPU usage)
        vTaskDelay(pdMS_TO_TICKS(50));
    }
}

/*
 * @brief Initialize GPIO pin as input
 * 
 * Configures the specified GPIO pin as an input with pull-down resistor.
 * When button is not pressed, pin will be LOW (0).
 * When button is pressed (connected to 3.3V), pin will be HIGH (1).
 */
 static void gpio_init(void)
 {
     // Configure GPIO pin structure
     gpio_config_t io_conf = {
         .intr_type = GPIO_INTR_DISABLE,      // No interrupt, we'll poll manually
         .mode = GPIO_MODE_INPUT,             // Set as input pin
         .pin_bit_mask = (1ULL << GPIO_BUTTON_PIN),  // Which pin to configure
         .pull_down_en = GPIO_PULLDOWN_ENABLE, // Enable pull-down resistor
         .pull_up_en = GPIO_PULLUP_DISABLE,    // Disable pull-up resistor
     };
     
     // Apply the configuration
     gpio_config(&io_conf);
     ESP_LOGI(TAG, "GPIO %d configured as input with pull-down", GPIO_BUTTON_PIN);
 }
 


static void mqtt_app_start(void)
{
    esp_mqtt_client_config_t mqtt_cfg = {
        .broker.address.uri = CONFIG_BROKER_URL,
    };
#if CONFIG_BROKER_URL_FROM_STDIN
    char line[128];

    if (strcmp(mqtt_cfg.broker.address.uri, "FROM_STDIN") == 0) {
        int count = 0;
        printf("Please enter url of mqtt broker\n");
        while (count < 128) {
            int c = fgetc(stdin);
            if (c == '\n') {
                line[count] = '\0';
                break;
            } else if (c > 0 && c < 127) {
                line[count] = c;
                ++count;
            }
            vTaskDelay(10 / portTICK_PERIOD_MS);
        }
        mqtt_cfg.broker.address.uri = line;
        printf("Broker url: %s\n", line);
    } else {
        ESP_LOGE(TAG, "Configuration mismatch: wrong broker url");
        abort();
    }
#endif /* CONFIG_BROKER_URL_FROM_STDIN */

    esp_mqtt_client_handle_t client = esp_mqtt_client_init(&mqtt_cfg);
    mqtt_client_handle = client;  // Store globally so GPIO task can use it
    /* The last argument may be used to pass data to the event handler, in this example mqtt_event_handler */
    esp_mqtt_client_register_event(client, ESP_EVENT_ANY_ID, mqtt_event_handler, NULL);
    esp_mqtt_client_start(client);
}

void app_main(void)
{
    ESP_LOGI(TAG, "[APP] Startup..");
    ESP_LOGI(TAG, "[APP] Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());
    ESP_LOGI(TAG, "[APP] IDF version: %s", esp_get_idf_version());

    esp_log_level_set("*", ESP_LOG_INFO);
    esp_log_level_set("mqtt_client", ESP_LOG_VERBOSE);
    esp_log_level_set("mqtt_example", ESP_LOG_VERBOSE);
    esp_log_level_set("transport_base", ESP_LOG_VERBOSE);
    esp_log_level_set("esp-tls", ESP_LOG_VERBOSE);
    esp_log_level_set("transport", ESP_LOG_VERBOSE);
    esp_log_level_set("outbox", ESP_LOG_VERBOSE);

    ESP_ERROR_CHECK(nvs_flash_init());
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());

    /* This helper function configures Wi-Fi or Ethernet, as selected in menuconfig.
     * Read "Establishing Wi-Fi or Ethernet Connection" section in
     * examples/protocols/README.md for more information about this function.
     */
    ESP_ERROR_CHECK(example_connect());

    // Initialize GPIO pin before starting MQTT
    gpio_init();

    // Initialize OpenAI API client
    const char *api_key = CONFIG_OPENAI_API_KEY;
    if (strlen(api_key) > 0) {
        openai_handle = OpenAICreate(api_key);
        if (openai_handle != NULL) {
            // Set custom API URL if configured (for OpenRouter, LM Studio, etc.)
            const char *api_url = CONFIG_OPENAI_API_URL;
            // Check if URL is different from default OpenAI URL
            if (strlen(api_url) > 0 && strcmp(api_url, "https://api.openai.com/v1/chat/completions") != 0) {
                // Extract base URL (remove /v1/chat/completions if present)
                char base_url[256];
                strncpy(base_url, api_url, sizeof(base_url) - 1);
                base_url[sizeof(base_url) - 1] = '\0';
                
                // Remove /v1/chat/completions suffix if present
                char *suffix = strstr(base_url, "/v1/chat/completions");
                if (suffix != NULL) {
                    *suffix = '\0';
                }
                
                OpenAIChangeBaseURL(openai_handle, base_url);
                ESP_LOGI(TAG, "OpenAI base URL set to: %s", base_url);
            }
            ESP_LOGI(TAG, "OpenAI API initialized successfully");
            ESP_LOGI(TAG, "Using model: %s", CONFIG_OPENAI_MODEL);
        } else {
            ESP_LOGE(TAG, "Failed to initialize OpenAI API");
        }
    } else {
        ESP_LOGW(TAG, "OpenAI API key not configured. ChatGPT features will be disabled.");
    }

    mqtt_app_start();

    // Create background task to monitor GPIO button
    // Parameters: function, task name, stack size, parameter, priority, task handle
    xTaskCreate(gpio_task, "gpio_task", 2048, NULL, 10, NULL);
    
    ESP_LOGI(TAG, "Application initialized. Monitoring GPIO %d for button presses...", GPIO_BUTTON_PIN);
    if (openai_handle != NULL) {
        ESP_LOGI(TAG, "ChatGPT integration ready. Press button to start endless discussion!");
    }
}
