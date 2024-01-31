// use core::fmt::Debug;
// use std::net::SocketAddr;
// use rustls::{Certificate, PrivateKey};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use tokio::net::{
    TcpListener,
    TcpStream,
    // UnixListener
};
// use tokio::signal;
// use tokio::time::timeout;
// Tls
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use rustls_pki_types::{
    CertificateDer,
    PrivateKeyDer,
    // PrivatePkcs1KeyDer
};
// use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::{
    rustls::ServerConfig, // TlsStream
    TlsAcceptor,
};

// traits
// use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{
    // AsyncRead,
    // AsyncWrite,
    ReadHalf,
    WriteHalf,
}; // for read_buf() / write()
   // use tokio_util::codec::{Decoder, Encoder}; // for encode() / decode()
   // use futures::StreamExt; // for next()

// Easy error handling with async code
type AResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Dbg

use rustls_pemfile::{read_one, Item};
use std::iter;

/*
 * certificate & key:
 *
 * cd certs && ./self_signed.sh && cd ..
 *
 * run:
 * cargo run -- 127.0.0.1:6161 certs/self_signed/server_cert.pem server_key_decrypted.pem
 *
 * test with:
 * openssl s_client -showcerts -connect 127.0.0.1:6161
 *
 * or with:
 * cargo run --examples client_self_signed 127.0.0.1:6161
 */

pub fn load_certs<P>(path: P) -> std::io::Result<Vec<CertificateDer<'static>>>
// -> std::io::Result<Vec<Certificate>>
where
    P: AsRef<Path>,
{
    certs(&mut BufReader::new(File::open(path.as_ref())?)).collect()
    // .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid certificate"))
    // .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

pub fn load_keys<P>(path: P) -> std::io::Result<Vec<PrivateKeyDer<'static>>>
where
    P: AsRef<Path>,
{
    // println!("Err0 ?? {:?}", File::open(path.as_ref()));
    // println!(
    //     "Err?? {:?}",
    //     rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?)).next()
    // );

    // rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?))
    //     .map(|k_| k_.and_then(|k| Ok(PrivateKeyDer::Pkcs1(k))))
    //     .collect()
    pkcs8_private_keys(&mut BufReader::new(File::open(path.as_ref())?))
        .map(|k_| k_.and_then(|k| Ok(PrivateKeyDer::Pkcs8(k))))
        .collect()

    // .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid key"))
    // .map(|k| PrivateKeyDer::Pkcs1(k))
    // .collect()
    // .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

async fn handle_conn(
    reader: &mut ReadHalf<tokio_rustls::server::TlsStream<TcpStream>>,
    writer: &mut WriteHalf<tokio_rustls::server::TlsStream<TcpStream>>,
    buffer_len: usize,
) {
    // same as handle_conn but with dynamic buffer_len + handle partial write

    // println!("Got a conn: {:?}");
    // let (mut reader, mut writer) = sock.split();

    let mut buffer: Vec<u8> = vec![0; buffer_len];

    loop {
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

        let s = std::str::from_utf8(&buffer[0..n]);
        let upper: String;
        let to_write = match s {
            Ok(s) => {
                upper = s.to_ascii_uppercase();
                upper.as_bytes()
            }
            Err(_) => &buffer[0..n],
        };

        let mut n_ = 0;
        let mut has_error = false;
        while !has_error {
            // Note: write returns n <= buf len so we ensures that we write the whole buffer
            match writer.write(&to_write[..]).await {
                Ok(nw) => {
                    n_ += nw;
                    if n_ == n {
                        break;
                    }
                }
                Err(e) => {
                    println!("Write error (after {} bytes): {}", n_, e);
                    has_error = true;
                    break;
                }
            }
        }
    }

    println!("End of coroutine: handle_conn...");
}

async fn serve() -> AResult<()> {
    // Skip args[0] (cmd line string) and only take first
    let arg: Vec<String> = env::args().skip(1).take(3).collect();

    // let empty_str = String::new();
    let (addr, cert, key, enable_tls) = match arg.len() {
        0 | 1 | 2 => panic!("Use: cargo run -- 127.0.0.1:7070 cert.pem key.pem"), // TODO: proper error
        3 => (&arg[0], &arg[1], &arg[2], true),
        _ => panic!("..."), // TODO: proper error
    };

    println!("arg 1: {}", cert);
    println!("arg 2: {}", key);
    // let enable_tls = !cert.is_empty() && !key.is_empty();
    println!("Enable tls: {}", enable_tls);

    {
        let mut reader = BufReader::new(File::open(key).unwrap());
        for item in iter::from_fn(|| read_one(&mut reader).transpose()) {
            match item.unwrap() {
                Item::X509Certificate(cert) => println!("certificate {:?}", cert),
                Item::Crl(crl) => println!("certificate revocation list: {:?}", crl),
                Item::Pkcs1Key(key) => println!("rsa pkcs1 key {:?}", key),
                Item::Pkcs8Key(key) => println!("pkcs8 key {:?}", key),
                Item::Sec1Key(key) => println!("sec1 ec key {:?}", key),
                _ => println!("unhandled item"),
            }
        }
    }

    let certs = load_certs(cert)?;
    println!("certs: {:?}", certs);
    let mut keys = load_keys(key)?;
    println!("keys: {:?}", keys);

    // let cert0 = certs.get(0).unwrap();
    // println!("cert0 len: {:?}", cert0.0.len());

    let config = ServerConfig::builder()
        // .with_safe_defaults()
        .with_no_client_auth()
        // .with_single_cert(certs, keys.remove(0))
        .with_single_cert(certs, keys.pop().ok_or("Unable to read key")?)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr[..]).await?;
    println!("[Tcp/Tls] Listening on {}", addr);
    loop {
        let (socket, _addr) = listener.accept().await?;
        // let acceptor_ = acceptor.clone();
        let mut stream = acceptor.accept(socket).await?;
        let (mut reader, mut writer) = tokio::io::split(stream);

        tokio::spawn(async move { handle_conn(&mut reader, &mut writer, 1024).await });
    }
}

async fn app_main() -> AResult<()> {
    println!("Starting tcp/tls server");
    serve().await
}

fn main() {
    // init the tokio async runtime - default is a multi threaded runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    // app_main func is our main entry point
    rt.block_on(app_main()).unwrap();
}
