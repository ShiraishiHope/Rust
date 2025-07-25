use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Deserialize, Serialize};
use std::io;

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

struct ProtocolClient {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
    authenticated: bool,
}

impl ProtocolClient {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect("127.0.0.1:8080").await?;
        let (reader, writer) = stream.into_split();
        let reader = BufReader::new(reader);
        
        Ok(ProtocolClient {
            reader,
            writer,
            authenticated: false,
        })
    }

    async fn send_message(&mut self, msg: Message) -> Result<Response, Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&msg)?;
        self.writer.write_all(format!("{}\n", json).as_bytes()).await?;
        
        let mut line = String::new();
        self.reader.read_line(&mut line).await?;
        let response: Response = serde_json::from_str(&line.trim())?;
        
        Ok(response)
    }

    async fn login(&mut self, username: String) -> Result<bool, Box<dyn std::error::Error>> {
        let response = self.send_message(Message::Login { username }).await?;
        
        match response {
            Response::LoginSuccess => {
                println!("Connexion réussie");
                self.authenticated = true;
                Ok(true)
            }
            Response::LoginError { reason } => {
                println!("Erreur: {}", reason);
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    async fn send_chat_message(&mut self, content: String) -> Result<(), Box<dyn std::error::Error>> {
        if !self.authenticated {
            println!("Non authentifié");
            return Ok(());
        }

        let response = self.send_message(Message::Message { content }).await?;
        
        match response {
            Response::MessageReceived => println!("Message envoyé"),
            Response::Error { message } => println!("Erreur: {}", message),
            _ => {}
        }
        
        Ok(())
    }

    async fn list_users(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_message(Message::List).await?;
        
        match response {
            Response::UserList { users } => {
                println!("Utilisateurs connectés:");
                for user in users {
                    println!("- {}", user);
                }
            }
            _ => {}
        }
        
        Ok(())
    }

    async fn logout(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.send_message(Message::Logout).await?;
        
        match response {
            Response::LogoutSuccess => {
                println!("Déconnecté");
                self.authenticated = false;
            }
            _ => {}
        }
        
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("=== CLIENT PROTOCOL ===");
        
        println!("Nom d'utilisateur:");
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        
        if !self.login(username.trim().to_string()).await? {
            return Ok(());
        }

        loop {
            println!("\nCommandes: message, list, logout");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
            
            match parts[0] {
                "message" => {
                    if parts.len() > 1 {
                        self.send_chat_message(parts[1].to_string()).await?;
                    } else {
                        println!("Usage: message <contenu>");
                    }
                }
                "list" => self.list_users().await?,
                "logout" => {
                    self.logout().await?;
                    break;
                }
                _ => println!("Commande inconnue"),
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProtocolClient::new().await?;
    client.run().await
}