use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const ADDRESS: &str = "127.0.0.1:8080";

struct content{
    username: String,
    data: String
}

fn content_to_bytes(content: &content) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(content.username.as_bytes());
    bytes.push(0); // Null byte as separator
    bytes.extend_from_slice(content.data.as_bytes());
    bytes
}

fn bytes_to_content(bytes: &[u8]) -> content {
    let parts: Vec<&[u8]> = bytes.splitn(2, |&b| b == 0).collect();
    let username = String::from_utf8(parts[0].to_vec()).unwrap_or_default();
    let data = if parts.len() > 1 {
        String::from_utf8(parts[1].to_vec()).unwrap_or_default()
    } else {
        String::new()
    };
    content { username, data }
}

async fn handle_client(mut socket: tokio::net::TcpStream, buf: &mut [u8; 1024]){

}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    println!("Server is running on {}", ADDRESS);

    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);

        tokio::spawn(async move{

        });
    }
}