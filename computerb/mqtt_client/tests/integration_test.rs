use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Incoming};
use std::env;
use std::time::Duration;
use tokio::time::timeout;

/// Integration test for ChatGPT API integration
/// 
/// This test requires:
/// - OPENAI_API_KEY or OPENROUTER_API_KEY environment variable
/// - Optionally OPENAI_API_BASE or OPENROUTER_BASE_URL for custom endpoints
/// 
/// Run with: cargo test --test integration_test -- --nocapture
#[tokio::test]
async fn test_chatgpt_api_integration() {
    // Get API key from environment
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
    
    println!("Testing ChatGPT API integration...");
    println!("Using endpoint: {}", endpoint);
    
    // Create OpenAI client with custom endpoint support
    let mut openai_client = if endpoint != "https://api.openai.com/v1" {
        OpenAIClient::builder()
            .with_endpoint(&endpoint)
            .with_api_key(api_key)
            .build()
            .expect("Failed to create OpenAI client")
    } else {
        OpenAIClient::builder()
            .with_api_key(api_key)
            .build()
            .expect("Failed to create OpenAI client")
    };
    
    // Create a simple test message
    let messages = vec![ChatCompletionMessage {
        role: MessageRole::user,
        content: Content::Text("Say 'Hello from integration test!' and nothing else.".to_string()),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }];
    
    // Create request
    let mut req = ChatCompletionRequest::new(
        model,
        messages,
    );
    req.temperature = Some(0.7);
    
    println!("Sending request to ChatGPT API...");
    
    // Call OpenAI API
    let result = openai_client
        .chat_completion(req)
        .await
        .expect("Failed to call OpenAI API");
    
    // Verify response
    assert!(!result.choices.is_empty(), "Response should contain at least one choice");
    
    if let Some(choice) = result.choices.first() {
        match &choice.message.content {
            Some(text) => {
                println!("✅ Success! ChatGPT response: {}", text);
                assert!(!text.is_empty(), "Response text should not be empty");
                // Check if response contains our expected phrase (case insensitive)
                let text_lower = text.to_lowercase();
                assert!(
                    text_lower.contains("hello") || text_lower.contains("integration"),
                    "Response should be relevant to our test"
                );
            }
            None => {
                panic!("Response content is None");
            }
        }
    } else {
        panic!("No choices in response");
    }
    
    println!("✅ Integration test passed!");
}

/// Test conversation history (multiple messages)
#[tokio::test]
async fn test_conversation_history() {
    // Get API key from environment
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
    
    println!("Testing conversation history...");
    println!("Using endpoint: {}", endpoint);
    println!("Using model: {}", model);
    
    // Create OpenAI client
    let mut openai_client = if endpoint != "https://api.openai.com/v1" {
        OpenAIClient::builder()
            .with_endpoint(&endpoint)
            .with_api_key(api_key)
            .build()
            .expect("Failed to create OpenAI client")
    } else {
        OpenAIClient::builder()
            .with_api_key(api_key)
            .build()
            .expect("Failed to create OpenAI client")
    };
    
    // Create conversation with multiple messages (simulating the endless discussion)
    let messages = vec![
        ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text("Write a very short story about a robot.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: MessageRole::assistant,
            content: Content::Text("Once upon a time, there was a robot named Rusty who loved to code.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text("Continue the story.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];
    
    // Create request
    let mut req = ChatCompletionRequest::new(
        model,
        messages,
    );
    req.temperature = Some(0.7);
    
    println!("Sending conversation to ChatGPT API...");
    
    // Call OpenAI API
    let result = openai_client
        .chat_completion(req)
        .await
        .expect("Failed to call OpenAI API");
    
    // Verify response
    assert!(!result.choices.is_empty(), "Response should contain at least one choice");
    
    if let Some(choice) = result.choices.first() {
        match &choice.message.content {
            Some(text) => {
                println!("✅ Success! ChatGPT continued the story: {}", text);
                assert!(!text.is_empty(), "Response text should not be empty");
                // The response should continue the story about the robot
                let text_lower = text.to_lowercase();
                assert!(
                    text_lower.len() > 10,
                    "Response should be substantial"
                );
            }
            None => {
                panic!("Response content is None");
            }
        }
    } else {
        panic!("No choices in response");
    }
    
    println!("✅ Conversation history test passed!");
}

/// Integration test for MQTT broker connectivity
/// 
/// This test verifies:
/// - Connection to MQTT broker
/// - Subscribing to a topic
/// - Publishing messages
/// - Receiving published messages
/// 
/// This test requires:
/// - Network connectivity to MQTT broker
/// - MQTT_BROKER environment variable (optional, defaults to broker.hivemq.com)
/// - MQTT_PORT environment variable (optional, defaults to 1883)
/// 
/// Run with: cargo test --test integration_test test_mqtt_broker -- --nocapture
#[tokio::test]
async fn test_mqtt_broker() {
    // Get MQTT broker configuration from environment (with defaults)
    let broker = env::var("MQTT_BROKER")
        .unwrap_or_else(|_| "broker.hivemq.com".to_string());
    
    let port: u16 = env::var("MQTT_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .expect("MQTT_PORT must be a valid port number");
    
    // Use a unique test topic to avoid conflicts
    let test_topic = format!("/test/mqtt_integration_{}", std::process::id());
    let test_message = "Hello from MQTT integration test!";
    
    println!("Testing MQTT broker connectivity...");
    println!("Broker: {}:{}", broker, port);
    println!("Test topic: {}", test_topic);
    
    // Create MQTT client options
    let client_id = format!("test_client_{}", std::process::id());
    let mut mqttoptions = MqttOptions::new(client_id, broker.clone(), port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    // Create async client and event loop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    
    // Subscribe to test topic
    println!("Subscribing to topic: {}", test_topic);
    client
        .subscribe(&test_topic, QoS::AtMostOnce)
        .await
        .expect("Failed to subscribe to test topic");
    
    // Wait a bit for subscription to be processed
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Publish test message
    println!("Publishing test message: {}", test_message);
    client
        .publish(&test_topic, QoS::AtMostOnce, false, test_message.as_bytes())
        .await
        .expect("Failed to publish test message");
    
    // Wait for the message to be received (with timeout)
    println!("Waiting for message to be received...");
    let receive_timeout = Duration::from_secs(10);
    let mut message_received = false;
    let mut connection_established = false;
    
    let start_time = std::time::Instant::now();
    
    loop {
        // Check timeout
        if start_time.elapsed() > receive_timeout {
            break;
        }
        
        // Poll for events with a short timeout
        match timeout(Duration::from_millis(500), eventloop.poll()).await {
            Ok(Ok(Event::Incoming(Incoming::ConnAck(_)))) => {
                println!("✅ Connected to MQTT broker");
                connection_established = true;
            }
            Ok(Ok(Event::Incoming(Incoming::Publish(publish)))) => {
                let payload = String::from_utf8_lossy(&publish.payload);
                println!("✅ Received message on topic '{}': {}", publish.topic, payload);
                
                // Verify it's our test message
                if publish.topic == test_topic && payload == test_message {
                    println!("✅ Message matches expected content!");
                    message_received = true;
                    break;
                }
            }
            Ok(Ok(Event::Incoming(Incoming::SubAck(_)))) => {
                println!("✅ Subscription acknowledged");
            }
            Ok(Ok(Event::Incoming(Incoming::PubAck(_)))) => {
                println!("✅ Publish acknowledged");
            }
            Ok(Err(e)) => {
                eprintln!("❌ MQTT error: {:?}", e);
                break;
            }
            Err(_) => {
                // Timeout on poll, continue waiting
                continue;
            }
            _ => {
                // Other events, continue
            }
        }
    }
    
    // Verify results
    assert!(
        connection_established,
        "Should have established connection to MQTT broker"
    );
    
    assert!(
        message_received,
        "Should have received the published test message"
    );
    
    println!("✅ MQTT broker integration test passed!");
}
