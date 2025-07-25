use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::net::Ipv4Addr;

struct DnsServer {
    domains: HashMap<String, Ipv4Addr>,
}

impl DnsServer {
    fn new() -> Self {
        let mut domains = HashMap::new();
        domains.insert("example.com".to_string(), Ipv4Addr::new(93, 184, 216, 34));
        domains.insert("google.com".to_string(), Ipv4Addr::new(142, 250, 185, 78));
        domains.insert("localhost".to_string(), Ipv4Addr::new(127, 0, 0, 1));
        
        DnsServer { domains }
    }

    fn parse_query(&self, data: &[u8]) -> Option<String> {
        if data.len() < 12 { return None; }
        
        let mut pos = 12;
        let mut domain = String::new();
        
        while pos < data.len() {
            let len = data[pos] as usize;
            if len == 0 { break; }
            
            pos += 1;
            if pos + len > data.len() { return None; }
            
            if !domain.is_empty() { domain.push('.'); }
            domain.push_str(&String::from_utf8_lossy(&data[pos..pos + len]));
            pos += len;
        }
        
        Some(domain)
    }

    fn create_response(&self, query: &[u8], domain: &str) -> Vec<u8> {
        let mut response = query[..12].to_vec();
        response[2] = 0x81;
        response[3] = 0x80;
        
        if let Some(ip) = self.domains.get(domain) {
            response[7] = 1;
            response.extend_from_slice(&query[12..]);
            
            response.extend_from_slice(&[0xc0, 0x0c]);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&[0x00, 0x01]);
            response.extend_from_slice(&[0x00, 0x00, 0x00, 0x3c]);
            response.extend_from_slice(&[0x00, 0x04]);
            response.extend_from_slice(&ip.octets());
        } else {
            response[3] = 0x83;
        }
        
        response
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("127.0.0.1:53").await?;
        println!("Serveur DNS démarré sur 127.0.0.1:53");
        
        let mut buf = [0; 512];
        
        loop {
            let (len, addr) = socket.recv_from(&mut buf).await?;
            
            if let Some(domain) = self.parse_query(&buf[..len]) {
                println!("Requête pour: {}", domain);
                let response = self.create_response(&buf[..len], &domain);
                socket.send_to(&response, addr).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = DnsServer::new();
    server.run().await
}