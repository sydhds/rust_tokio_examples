use std::error::Error;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

async fn handle_conn(mut sock: TcpStream, buffer_len: usize) {
    // same as handle_conn but with dynamic buffer_len + handle partial write

    println!("Got a connection: {:?}", sock);
    let (mut reader, mut writer) = sock.split();

    let mut buffer: Vec<u8> = vec![0; buffer_len];

    loop {
        // Read bytes into buffer
        let n = match reader.read(&mut buffer[..]).await {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        };

        if n == 0 {
            println!("0 bytes read, aborting...");
            break;
        }

        // Then bytes -> str -> str + uppercase characters
        let s = str::from_utf8(&buffer[0..n]);
        let upper: String;
        let to_write = match s {
            Ok(s) => {
                upper = s.to_ascii_uppercase();
                upper.as_bytes()
            }
            Err(_) => &buffer[0..n],
        };

        // And then write back to our client
        // Note: This method is not cancellation safe. Do not use in tokio::select!
        if let Err(e) = writer.write_all(&to_write[..]).await {
            println!("Write error: {}", e);
            break;
        }
    }

    println!("End of coroutine: handle_conn...");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:6161";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (sock, _) = listener.accept().await?;

        // Spawn a task to handle this connection
        tokio::spawn(async move {
            handle_conn(sock, 1024).await;
        });
    }
}
