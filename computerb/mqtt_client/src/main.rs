use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Incoming};
use std::time::Duration;
use std::env;
use std::collections::VecDeque;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};

// Maximum conversation history to prevent unbounded growth
const MAX_CONVERSATION_HISTORY: usize = 10;
const MAX_MESSAGE_LENGTH: usize = 500;

/// Truncate message if too long
fn truncate_message(msg: &str, max_len: usize) -> String {
    if msg.len() <= max_len {
        msg.to_string()
    } else {
        format!("{}...", &msg[..max_len - 3])
    }
}

/// Call OpenAI API with conversation history
async fn call_openai_api(
    messages: Vec<ChatCompletionMessage>,
    client: &mut OpenAIClient,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut req = ChatCompletionRequest::new(
        model.to_string(),
        messages,
    );
    req.temperature = Some(0.7);
    
    let result = client.chat_completion(req).await?;
    
    if let Some(choice) = result.choices.first() {
        match &choice.message.content {
            Some(text) => Ok(text.clone()),
            None => Err("Empty response from OpenAI API".into()),
        }
    } else {
        Err("No response from OpenAI API".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting MQTT client with ChatGPT integration...");
    
    // Get OpenAI API configuration from environment
    let api_key = env::var("OPENAI_API_KEY")
        .or_else(|_| env::var("OPENROUTER_API_KEY"))
        .expect("OPENAI_API_KEY or OPENROUTER_API_KEY environment variable must be set");
    
    // Get custom endpoint if specified (defaults to OpenRouter)
    let endpoint = env::var("OPENAI_API_BASE")
        .or_else(|_| env::var("OPENROUTER_BASE_URL"))
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());
    
    // Get model name (defaults to free OpenRouter model)
    let model = env::var("OPENAI_MODEL")
        .or_else(|_| env::var("OPENROUTER_MODEL"))
        .unwrap_or_else(|_| "x-ai/grok-4.1-fast".to_string());
    
    // Create OpenAI client with custom endpoint support
    let mut openai_client = if endpoint != "https://api.openai.com/v1" {
        OpenAIClient::builder()
            .with_endpoint(&endpoint)
            .with_api_key(api_key)
            .build()?
    } else {
        OpenAIClient::builder()
            .with_api_key(api_key)
            .build()?
    };
    
    println!("OpenAI API client initialized");
    println!("Using endpoint: {}", endpoint);
    println!("Using model: {}", model);
    
    // MQTT broker configuration
    let broker = "broker.hivemq.com";
    let port = 1883;
    let client_id = "rust_chatgpt_client";
    let subscribe_topic = "/esp_gpt_out";  // Subscribe to ESP32's ChatGPT responses
    let publish_topic = "/client_gpt";     // Publish our ChatGPT responses to ESP32
    
    // Create MQTT client
    let mut mqttoptions = MqttOptions::new(client_id, broker, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    // Create async client and event loop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // Subscribe to /esp_gpt_out topic to receive ChatGPT responses from ESP32
    client.subscribe(subscribe_topic, QoS::AtMostOnce).await?;
    println!("Subscribed to topic: {} (receiving ChatGPT responses from ESP32)", subscribe_topic);
    println!("Will publish to topic: {} (sending ChatGPT responses to ESP32)", publish_topic);
    println!("Waiting for messages to start endless discussion...");
    
    // Conversation history (maintained by Rust client)
    let mut conversation_history: VecDeque<ChatCompletionMessage> = VecDeque::new();
    
    // Event loop - wait for messages
    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                let payload = String::from_utf8_lossy(&publish.payload);
                println!("[RECEIVED] Topic: '{}' | Message: '{}'", publish.topic, payload);
                
                // Check if this is a message from /esp_gpt_out (ChatGPT response from ESP32)
                if publish.topic == subscribe_topic {
                    let esp32_response = truncate_message(&payload, MAX_MESSAGE_LENGTH);
                    
                    // Add ESP32's ChatGPT response to conversation history as "assistant"
                    conversation_history.push_back(ChatCompletionMessage {
                        role: MessageRole::assistant,
                        content: Content::Text(esp32_response.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    });
                    
                    // Trim conversation history if too long (remove oldest messages)
                    while conversation_history.len() > MAX_CONVERSATION_HISTORY {
                        conversation_history.pop_front();
                    }
                    
                    println!("[CHATGPT] ESP32 said: {}", esp32_response);
                    println!("[CHATGPT] Calling OpenAI API to continue conversation...");
                    
                    // Convert VecDeque to Vec for API call
                    let messages: Vec<ChatCompletionMessage> = conversation_history
                        .iter()
                        .map(|msg| {
                            match &msg.content {
                                Content::Text(text) => ChatCompletionMessage {
                                    role: msg.role.clone(),
                                    content: Content::Text(text.clone()),
                                    name: msg.name.clone(),
                                    tool_calls: msg.tool_calls.clone(),
                                    tool_call_id: msg.tool_call_id.clone(),
                                },
                                _ => {
                                    // For non-text content, try to clone or create a new message
                                    ChatCompletionMessage {
                                        role: msg.role.clone(),
                                        content: msg.content.clone(),
                                        name: msg.name.clone(),
                                        tool_calls: msg.tool_calls.clone(),
                                        tool_call_id: msg.tool_call_id.clone(),
                                    }
                                }
                            }
                        })
                        .collect();
                    
                    // Call OpenAI API with conversation history
                    match call_openai_api(messages, &mut openai_client, &model).await {
                        Ok(response) => {
                            let truncated_response = truncate_message(&response, MAX_MESSAGE_LENGTH);
                            
                            // Add our ChatGPT response to conversation history as "assistant"
                            conversation_history.push_back(ChatCompletionMessage {
                                role: MessageRole::assistant,
                                content: Content::Text(truncated_response.clone()),
                                name: None,
                                tool_calls: None,
                                tool_call_id: None,
                            });
                            
                            // Trim again if needed
                            while conversation_history.len() > MAX_CONVERSATION_HISTORY {
                                conversation_history.pop_front();
                            }
                            
                            println!("[CHATGPT] Response: {}", truncated_response);
                            
                            // Publish ChatGPT response to /client_gpt topic
                            match client.publish(
                                publish_topic,
                                QoS::AtMostOnce,
                                false,
                                truncated_response.as_bytes(),
                            ).await {
                                Ok(_) => {
                                    println!("[PUBLISHED] Sent ChatGPT response to ESP32 via {}", publish_topic);
                                }
                                Err(e) => {
                                    eprintln!("[ERROR] Failed to publish to {}: {}", publish_topic, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[ERROR] OpenAI API call failed: {}", e);
                            // Continue listening for next message
                        }
                    }
                }
            }
            Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                println!("[CONNECTED] Connected to MQTT broker: {}", broker);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR] MQTT error: {:?}", e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
