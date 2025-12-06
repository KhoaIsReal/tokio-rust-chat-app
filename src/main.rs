use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

const ADDRESS: &str = "127.0.0.1:8080";

struct Content {
    username: String,
    data: String,
}

fn content_to_bytes(content: &Content) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(content.username.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(content.data.as_bytes());
    bytes
}

fn bytes_to_content(bytes: &[u8]) -> Content {
    let parts: Vec<&[u8]> = bytes.splitn(2, |&b| b == 0).collect();
    let username = String::from_utf8_lossy(parts[0]).to_string();
    let data = if parts.len() > 1 {
        String::from_utf8_lossy(parts[1]).to_string()
    } else {
        String::new()
    };
    Content { username, data }
}

async fn broadcast_message(
    message: &Content,
    clients: &Arc<Mutex<Vec<tokio::net::tcp::OwnedWriteHalf>>>,
) {
    let bytes = content_to_bytes(message);

    let mut guard = clients.lock().await;

    for writer in guard.iter_mut() {
        let _ = writer.write_all(&bytes).await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();
    println!("Server running on {}", ADDRESS);

    let clients = Arc::new(Mutex::new(Vec::<tokio::net::tcp::OwnedWriteHalf>::new()));

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection: {}", addr);

        let clients_clone = clients.clone();

        tokio::spawn(async move {
            // split stream
            let (mut reader, writer) = socket.into_split();

            // save writer for broadcast
            {
                let mut list = clients_clone.lock().await;
                list.push(writer);
            }

            let mut buf = vec![0u8; 1024];

            loop {
                let n = match reader.read(&mut buf).await {
                    Ok(0) => {
                        println!("{} disconnected", addr);
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("read error from {}: {}", addr, e);
                        break;
                    }
                };

                let msg = bytes_to_content(&buf[..n]);

                println!("{}: {}", msg.username, msg.data);

                broadcast_message(&msg, &clients_clone).await;
            }
        });
    }
}
