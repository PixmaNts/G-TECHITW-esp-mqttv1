use rumqttc::{MqttOptions, Client, QoS, Event, Incoming};
use std::time::Duration;

fn main() {
    println!("Starting MQTT subscriber...");
    
    // MQTT broker configuration
    let broker = "broker.hivemq.com";
    let port = 1883;
    let client_id = "rust_subscriber";
    let topic = "/esp32_gpio";

    // Create MQTT client
    let mut mqttoptions = MqttOptions::new(client_id, broker, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    // Create client and event loop
    let (client, mut eventloop) = Client::new(mqttoptions, 10);

    // Subscribe to topic
    client.subscribe(topic, QoS::AtMostOnce).unwrap();
    println!("Subscribed to topic: {}", topic);
    println!("Waiting for messages from ESP32...");

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