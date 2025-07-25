use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::io;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: String,
}

struct WebSocketClient {
    username: String,
}

impl WebSocketClient {
    fn new(username: String) -> Self {
        WebSocketClient { username }
    }

    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "ws://127.0.0.1:8080";
        let (ws_stream, _) = connect_async(url).await?;
        println!("Connecté au serveur WebSocket");
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        let username = self.username.clone();
        tokio::spawn(async move {
            let mut input = String::new();
            loop {
                input.clear();
                if io::stdin().read_line(&mut input).is_err() {
                    break;
                }
                
                let content = input.trim();
                if content == "/quit" {
                    break;
                }
                
                if !content.is_empty() {
                    let chat_msg = ChatMessage {
                        username: username.clone(),
                        content: content.to_string(),
                        timestamp: Utc::now().format("%H:%M:%S").to_string(),
                    };
                    
                    if let Ok(json) = serde_json::to_string(&chat_msg) {
                        if ws_sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
            }
            
            let _ = ws_sender.send(Message::Close(None)).await;
        });
        
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                        println!("[{}] {}: {}", chat_msg.timestamp, chat_msg.username, chat_msg.content);
                    }
                }
                Ok(Message::Close(_)) => {
                    println!("Connexion fermée");
                    break;
                }
                Err(e) => {
                    println!("Erreur: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        Ok(())
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== CLIENT WEBSOCKET ===");
        println!("Connecté en tant que: {}", self.username);
        println!("Tapez vos messages (ou /quit pour quitter)");
        
        self.connect().await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Nom d'utilisateur:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    
    let client = WebSocketClient::new(username.trim().to_string());
    client.run().await
}