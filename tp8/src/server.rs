use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Message {
    Login { username: String },
    Message { content: String },
    List,
    Logout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Response {
    LoginSuccess,
    LoginError { reason: String },
    MessageReceived,
    UserList { users: Vec<String> },
    LogoutSuccess,
    Error { message: String },
}

struct ProtocolServer {
    clients: Arc<Mutex<HashMap<String, String>>>,
}

impl ProtocolServer {
    fn new() -> Self {
        ProtocolServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn handle_client(&self, stream: TcpStream, addr: String) {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();
        let mut current_user: Option<String> = None;

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    if let Ok(msg) = serde_json::from_str::<Message>(&line.trim()) {
                        let response = self.process_message(msg, &mut current_user, &addr).await;
                        let response_json = serde_json::to_string(&response).unwrap();
                        
                        if writer.write_all(format!("{}\n", response_json).as_bytes()).await.is_err() {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }

        if let Some(username) = current_user {
            self.clients.lock().await.remove(&username);
        }
    }

    async fn process_message(&self, msg: Message, current_user: &mut Option<String>, addr: &str) -> Response {
        match msg {
            Message::Login { username } => {
                let mut clients = self.clients.lock().await;
                if clients.contains_key(&username) {
                    Response::LoginError { reason: "Utilisateur déjà connecté".to_string() }
                } else {
                    clients.insert(username.clone(), addr.to_string());
                    *current_user = Some(username.clone());
                    println!("Connexion: {}", username);
                    Response::LoginSuccess
                }
            }
            Message::Message { content } => {
                if let Some(user) = current_user {
                    println!("[{}]: {}", user, content);
                    Response::MessageReceived
                } else {
                    Response::Error { message: "Non authentifié".to_string() }
                }
            }
            Message::List => {
                let clients = self.clients.lock().await;
                let users: Vec<String> = clients.keys().cloned().collect();
                Response::UserList { users }
            }
            Message::Logout => {
                if let Some(username) = current_user.take() {
                    self.clients.lock().await.remove(&username);
                    println!("Déconnexion: {}", username);
                }
                Response::LogoutSuccess
            }
        }
    }

    async fn run(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Serveur démarré sur 127.0.0.1:8080");

        loop {
            let (stream, addr) = listener.accept().await?;
            let server_clone = Arc::clone(&self);
            
            tokio::spawn(async move {
                server_clone.handle_client(stream, addr.to_string()).await;
            });
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Arc::new(ProtocolServer::new());
    server.run().await
}