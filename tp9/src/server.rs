use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: String,
}

#[derive(Clone)]
struct WebSocketServer {
    clients: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>>,
}

impl WebSocketServer {
    fn new() -> Self {
        WebSocketServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn handle_connection(&self, stream: TcpStream, addr: String) {
        if let Ok(ws_stream) = accept_async(stream).await {
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            let (tx, mut rx) = mpsc::unbounded_channel();
            
            let clients = self.clients.clone();
            let client_id = format!("client_{}", addr);
            
            clients.lock().await.insert(client_id.clone(), tx);
            println!("Client connecté: {}", client_id);
            
            let clients_clone = clients.clone();
            let client_id_clone = client_id.clone();
            
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if ws_sender.send(msg).await.is_err() {
                        break;
                    }
                }
            });
            
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                            println!("[{}]: {}", chat_msg.username, chat_msg.content);
                            self.broadcast_message(&chat_msg, &client_id).await;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
            
            clients_clone.lock().await.remove(&client_id_clone);
            println!("Client déconnecté: {}", client_id_clone);
        }
    }

    async fn broadcast_message(&self, msg: &ChatMessage, sender_id: &str) {
        let clients = self.clients.lock().await;
        let json = serde_json::to_string(msg).unwrap();
        let message = Message::Text(json);
        
        for (client_id, tx) in clients.iter() {
            if client_id != sender_id {
                let _ = tx.send(message.clone());
            }
        }
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Serveur WebSocket démarré sur ws://127.0.0.1:8080");
        
        while let Ok((stream, addr)) = listener.accept().await {
            let server = self.clone();
            tokio::spawn(async move {
                server.handle_connection(stream, addr.to_string()).await;
            });
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = WebSocketServer::new();
    server.run().await
}