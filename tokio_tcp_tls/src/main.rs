use core::fmt::Debug;
use std::net::SocketAddr;
use std::env;
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

use tokio::net::{TcpListener, TcpStream, UnixListener};
use tokio::signal;
use tokio::time::timeout;
// Tls
use tokio_rustls::{TlsAcceptor, TlsStream, webpki};
use rustls_pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use rustls::{DistinguishedNames, OwnedTrustAnchor, RootCertStore};
use rustls::server::{AllowAnyAuthenticatedClient, ClientCertVerified, ClientCertVerifier};

// traits
// use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::io::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use x509_parser::certificate::X509Certificate;
use x509_parser::parse_x509_certificate;
use x509_parser::x509::X509Name;

// Easy error handling with async code
type AResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/*
 * run:
 *
 * test with:
 *
 * or with:
 *
 */

pub fn load_certs<P>(path: P) -> std::io::Result<Vec<Certificate>> where P: AsRef<Path> {
    certs(&mut BufReader::new(File::open(path.as_ref())?))
        .map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid certificate")
        })
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

pub fn load_keys<P>(path: P) -> std::io::Result<Vec<PrivateKey>> where P: AsRef<Path> {

    // println!("{:?}", rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?)));
    rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?))
        .map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid key")
        })
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

// client cert verifier

/*
type CertChainAndRoots<'a, 'b> = (
    webpki::EndEntityCert<'a>,
    Vec<&'a [u8]>,
    Vec<webpki::TrustAnchor<'b>>,
);

fn pki_error(error: webpki::Error) -> rustls::Error {
    use webpki::Error::*;
    match error {
        BadDer | BadDerTime => rustls::Error::InvalidCertificateEncoding,
        InvalidSignatureForPublicKey => rustls::Error::InvalidCertificateSignature,
        UnsupportedSignatureAlgorithm | UnsupportedSignatureAlgorithmForPublicKey => {
            rustls::Error::InvalidCertificateSignatureType
        }
        e => rustls::Error::InvalidCertificateData(format!("invalid peer certificate: {}", e)),
    }
}

fn prepare<'a, 'b>(
    end_entity: &'a Certificate,
    intermediates: &'a [Certificate],
    roots: &'b RootCertStore,
) -> Result<CertChainAndRoots<'a, 'b>, rustls::Error> {
    // EE cert must appear first.
    let cert = webpki::EndEntityCert::try_from(end_entity.0.as_ref())
        .map_err(pki_error)?; // FIXME - get back to original code

    let intermediates: Vec<&'a [u8]> = intermediates
        .iter()
        .map(|cert| cert.0.as_ref())
        .collect();

    let trustroots: Vec<webpki::TrustAnchor> = roots
        .roots
        .iter()
        // .map(OwnedTrustAnchor::to_trust_anchor)
        .map(|a| a.as_ref())
        .collect();

    Ok((cert, intermediates, trustroots))
}

type SignatureAlgorithms = &'static [&'static webpki::SignatureAlgorithm];
static SUPPORTED_SIG_ALGS: SignatureAlgorithms = &[
    &webpki::ECDSA_P256_SHA256,
    &webpki::ECDSA_P256_SHA384,
    &webpki::ECDSA_P384_SHA256,
    &webpki::ECDSA_P384_SHA384,
    &webpki::ED25519,
    &webpki::RSA_PSS_2048_8192_SHA256_LEGACY_KEY,
    &webpki::RSA_PSS_2048_8192_SHA384_LEGACY_KEY,
    &webpki::RSA_PSS_2048_8192_SHA512_LEGACY_KEY,
    &webpki::RSA_PKCS1_2048_8192_SHA256,
    &webpki::RSA_PKCS1_2048_8192_SHA384,
    &webpki::RSA_PKCS1_2048_8192_SHA512,
    &webpki::RSA_PKCS1_3072_8192_SHA384,
];
*/

fn get_first_cn_as_str<'a>(name: &'a X509Name<'_>) -> Option<&'a str> {
    name.iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
}

struct AllowAuthenticatedClient {
    roots: RootCertStore,
}

impl AllowAuthenticatedClient {
    /// Construct a new `AllowAnyAuthenticatedClient`.
    ///
    /// `roots` is the list of trust anchors to use for certificate validation.
    pub fn new(roots: RootCertStore) -> Arc<dyn ClientCertVerifier> {
        Arc::new(Self { roots })
    }
}

impl ClientCertVerifier for AllowAuthenticatedClient {
    fn offer_client_auth(&self) -> bool {
        true
    }

    fn client_auth_root_subjects(&self) -> Option<DistinguishedNames> {
        Some(self.roots.subjects())
    }

    fn verify_client_cert(
        &self,
        end_entity: &Certificate,
        intermediates: &[Certificate],
        now: SystemTime,
    ) -> Result<ClientCertVerified, rustls::Error> {

        /*
        let (cert, chain, trustroots) = prepare(end_entity, intermediates, &self.roots)?;
        let now = webpki::Time::try_from(now).map_err(|_| rustls::Error::FailedToGetCurrentTime)?;
        println!("Client cert verifying...");
        cert.verify_is_valid_tls_client_cert(
            SUPPORTED_SIG_ALGS,
            &webpki::TlsClientTrustAnchors(&trustroots),
            &chain,
            now,
        )
            .map_err(pki_error)
            .map(|_| ClientCertVerified::assertion())

        */

        let client_auth = rustls::server::AllowAnyAuthenticatedClient::new(
            self.roots.clone());

        println!("Client cert verifying...");
        let res = client_auth.verify_client_cert(end_entity, intermediates, now);

        if res.is_ok() {
            // Verify name

            let x509_cert: X509Certificate = parse_x509_certificate(&end_entity.0)
                .map_err(|_|
                    rustls::Error::InvalidCertificateSignature
                )?
                .1;
            // println!("x509_cert: {:?}", x509_cert);
            // println!("x509_cert: {:?}", get_first_cn_as_str(x509_cert.subject()));
            // println!("x509_cert: {:?}", x509_cert.name_constraints());

            let first_cn_name = get_first_cn_as_str(x509_cert.subject());


            first_cn_name
                .map(|cn_name|
                    if cn_name == "John Doe" {
                        cn_name
                    } else {
                        None
                    }
                )
                .ok_or(rustls::Error::InvalidCertificateSignature)?;
            // println!("first_cn_name: {:?}", first_cn_name);
        }

        res

    }
}

async fn handle_conn(reader: &mut ReadHalf<tokio_rustls::server::TlsStream<TcpStream>>,
                     writer: &mut WriteHalf<tokio_rustls::server::TlsStream<TcpStream>>,
                     buffer_len: usize)
{

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
            },
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
            },
            Err(_) => {
                &buffer[0..n]
            }
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
                },
                Err(e) => {
                    println!("Write error (after {} bytes): {}", n_, e);
                    has_error = true;
                    break;
                },
            }
        }
    }

    println!("End of coroutine: handle_conn...");
}

async fn serve() -> AResult<()> {

    // Skip args[0] (cmd line string) and only take first
    let arg: Vec<String> = env::args().skip(1).take(3).collect();

    let empty_str = String::new();
    let (addr, cert, key, enable_tls) = match arg.len() {
        0 | 1 | 2 => panic!("Use: cargo run -- 127.0.0.1:7070 cert.pem key.pem"), // TODO: proper error
        3 => (&arg[0], &arg[1], &arg[2], true),
        _ => panic!("..."), // TODO: proper error
    };

    println!("arg 1: {}", cert);
    println!("arg 2: {}", key);
    // let enable_tls = !cert.is_empty() && !key.is_empty();
    println!("Enable tls: {}", enable_tls);

    let certs = load_certs(cert)?;
    let mut keys = load_keys(key)?;

    println!("certs: {:?}", certs);
    let cert0 = certs.get(0).unwrap();
    println!("cert0 len: {:?}", cert0.0.len());
    println!("keys len: {}", keys.len());

    let mut root_cert_store = rustls::RootCertStore::empty();
    let mut pem = BufReader::new(File::open("certs/ca_signed_client_auth/root_ca.pem")?);
    let c_certs = rustls_pemfile::certs(&mut pem)?;
    let trust_anchors = c_certs.iter().map(|cert| {
        let ta = webpki::TrustAnchor::try_from_cert_der(&cert[..]).unwrap();
        println!("name const: {:?}", ta.name_constraints);
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    });
    root_cert_store.add_server_trust_anchors(trust_anchors);
    // let client_auth = rustls::server::AllowAnyAuthenticatedClient::new(
    //     root_cert_store);
    let client_auth1 = AllowAuthenticatedClient::new(root_cert_store);

    println!("yoyo");
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        // .with_no_client_auth()
        .with_client_cert_verifier(client_auth1)
        .with_single_cert(certs, keys.pop().ok_or("Unable to read key")?)
        .map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
        })?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(&addr[..]).await?;
    println!("[Tcp/Tls] Listening on {}", addr);
    loop {
        let (socket, _addr) = listener.accept().await?;
        let acceptor_ = acceptor.clone();
        let mut stream = acceptor.accept(socket).await?;
        let (mut reader, mut writer) = tokio::io::split(stream);

        tokio::spawn(async move {
            /*
            if let Err(e) = tunnel_stream(reader, writer, resolver_).await {
                println!("[Tcp/Tls] Tunnel stream error: {}", e);
            }
            */
            handle_conn(&mut reader, &mut writer, 1024).await
        });
    }
}

async fn app_main() -> AResult<()> {
    println!("Starting tcp/tls server");
    serve().await
    // Ok(())
}

fn main() {

    // init the tokio async runtime - default is a multi threaded runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    // app_main func is our main entry point
    // TODO: handle Result and return a integer (like a regular linux cmd)
    if let Err(e) = rt.block_on(app_main()) {
        println!("Error: {}", e);
    }
}
