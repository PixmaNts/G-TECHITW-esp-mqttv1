use rumqttc::{MqttOptions, Client, QoS, Event, Incoming};
use std::time::Duration;
use std::env;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let message_to_publish = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    println!("Starting MQTT client...");
    
    // MQTT broker configuration
    let broker = "broker.hivemq.com";
    let port = 1883;
    let client_id = "rust_subscriber";
    let subscribe_topic = "/esp32_gpio";
    let publish_topic = "/esp32_commands";

    // Create MQTT client
    let mut mqttoptions = MqttOptions::new(client_id, broker, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // Create client and event loop
    let (client, mut eventloop) = Client::new(mqttoptions, 10);

    // Subscribe to topic to receive button press messages
    client.subscribe(subscribe_topic, QoS::AtMostOnce).unwrap();
    println!("Subscribed to topic: {}", subscribe_topic);

    // If a message was provided as argument, publish it to ESP32
    if let Some(msg) = message_to_publish {
        println!("Publishing message to {}: {}", publish_topic, msg);
        client.publish(publish_topic, QoS::AtMostOnce, false, msg.as_bytes()).unwrap();
        println!("Message published successfully!");
    } else {
        println!("No message provided. Usage: cargo run -- \"<message>\"");
        println!("Waiting for messages from ESP32...");
    }

    // Event loop - wait for messages
    loop {
        match eventloop.recv().unwrap() {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                let payload = String::from_utf8_lossy(&publish.payload);
                println!("[RECEIVED] Topic: '{}' | Message: '{}'", publish.topic, payload);
            }
            Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                println!("[CONNECTED] Connected to MQTT broker: {}", broker);
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}