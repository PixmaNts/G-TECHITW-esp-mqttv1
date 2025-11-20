use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, MessageRole, Content};
use std::env;

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
        "x-ai/grok-4.1-fast".to_string(),
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
