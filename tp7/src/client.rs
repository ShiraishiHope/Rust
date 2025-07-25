use tokio::net::UdpSocket;
use std::net::Ipv4Addr;
use std::io;

struct DnsClient {
    socket: UdpSocket,
}

impl DnsClient {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(DnsClient { socket })
    }

    fn create_query(&self, domain: &str) -> Vec<u8> {
        let mut query = Vec::new();
        
        query.extend_from_slice(&[0x12, 0x34]);
        query.extend_from_slice(&[0x01, 0x00]);
        query.extend_from_slice(&[0x00, 0x01]);
        query.extend_from_slice(&[0x00, 0x00]);
        query.extend_from_slice(&[0x00, 0x00]);
        query.extend_from_slice(&[0x00, 0x00]);
        
        for part in domain.split('.') {
            query.push(part.len() as u8);
            query.extend_from_slice(part.as_bytes());
        }
        query.push(0);
        
        query.extend_from_slice(&[0x00, 0x01]);
        query.extend_from_slice(&[0x00, 0x01]);
        
        query
    }

    fn parse_response(&self, data: &[u8]) -> Option<Ipv4Addr> {
        if data.len() < 12 { return None; }
        
        let answers = u16::from_be_bytes([data[6], data[7]]);
        if answers == 0 { return None; }
        
        let mut pos = 12;
        while pos < data.len() && data[pos] != 0 {
            let len = data[pos] as usize;
            pos += len + 1;
        }
        pos += 5;
        
        while pos < data.len() {
            if pos + 1 < data.len() && data[pos] == 0xc0 {
                pos += 2;
                if pos + 10 <= data.len() {
                    let data_len = u16::from_be_bytes([data[pos + 8], data[pos + 9]]) as usize;
                    if data_len == 4 && pos + 10 + data_len <= data.len() {
                        return Some(Ipv4Addr::new(
                            data[pos + 10],
                            data[pos + 11],
                            data[pos + 12],
                            data[pos + 13],
                        ));
                    }
                }
                break;
            }
            pos += 1;
        }
        
        None
    }

    async fn resolve(&self, domain: &str) -> Result<Option<Ipv4Addr>, Box<dyn std::error::Error>> {
        let query = self.create_query(domain);
        self.socket.send_to(&query, "127.0.0.1:53").await?;
        
        let mut buf = [0; 512];
        let (len, _) = self.socket.recv_from(&mut buf).await?;
        
        Ok(self.parse_response(&buf[..len]))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DnsClient::new().await?;
    
    loop {
        println!("Domaine à résoudre (ou 'quit'):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let domain = input.trim();
        
        if domain == "quit" { break; }
        
        match client.resolve(domain).await? {
            Some(ip) => println!("{} -> {}", domain, ip),
            None => println!("Domaine non trouvé"),
        }
    }
    
    Ok(())
}