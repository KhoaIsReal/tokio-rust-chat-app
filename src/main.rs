use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const ADDRESS: &str = "127.0.0.1:8080";

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