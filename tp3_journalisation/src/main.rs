//Imports listener + connexion
use tokio::net::{TcpListener, TcpStream};
//read_line() & write_all() asynchrone + opti lecture par blocks
use tokio::io::{AsyncBufReadExt,AsyncWriteExt, BufReader};
//ouvrir fichiers avec des options spécifiques
use tokio::fs::OpenOptions;
//datetime
use chrono::Utc;
//partage de données sécurisé entre threads
use std::sync::Arc;
//Verrou compatible asynchrone
use tokio::sync::Mutex;

//Structure de la journalisation
struct LogServer {
    //plusieurs taches peuvent écrire dans le fichier log, mais une à la fois
    log_file: Arc<Mutex<tokio::fs::File>>,
}


impl LogServer {
    //constructeur
    //création du dossier logs/
    //ouverture du server.log en mode append
    //encapsule via Arc<Mutex> pour un partage sécurisé
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all("logs").await?;
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/server.log")
            .await?;
            
        Ok(LogServer {
            log_file: Arc::new(Mutex::new(file)),
        })
    }

    //génération du datetime
    //formatage des écritures et ecriture dans le fichier + console
    async fn log_message(&self, client_addr: &str, message: &str) {
        let timestamp = Utc::now().format("[%Y-%m-%dT%H:%M:%S]");
        let log_entry = format!("{} [{}]: {}\n", timestamp, client_addr, message.trim());
        
        let mut file = self.log_file.lock().await;
        if let Err(e) = file.write_all(log_entry.as_bytes()).await {
            eprintln!("Erreur écriture log: {}", e);
        }
        
        println!("{} [{}]: {}", timestamp, client_addr, message.trim());
    }

    //utilisation de TcpStream dans BufReader pour optimisation de la lecture
    //loop: lecture ligne par ligne jusqua deconnexion
    //Journalisation co/deco + tous les messages
    async fn handle_client(&self, mut stream: TcpStream, addr: String) {
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();

        self.log_message(&addr, "Client connecté").await;

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    self.log_message(&addr, &line).await;
                }
                Err(e) => {
                    eprintln!("Erreur lecture: {}", e);
                    break;
                }
            }
        }

        self.log_message(&addr, "Client déconnecté").await;
    }
}

//création du LogServer et encapsulation dans Arc
//Bind sur 127.0.0.1:8080
//boucle: accept() + spawn() pour chaque client (tache indé avec clone d'Arc)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = LogServer::new().await?;
    let server = Arc::new(server);
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur démarré sur 127.0.0.1:8080");

    loop {
        let (stream, addr) = listener.accept().await?;
        let server_clone = Arc::clone(&server);
        
        tokio::spawn(async move {
            server_clone.handle_client(stream, addr.to_string()).await;
        });
    }
}