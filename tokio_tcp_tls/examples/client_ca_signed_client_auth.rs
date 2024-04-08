use std::error::Error;
use std::sync::Arc;
// use std::time::SystemTime;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
// use std::path::Path;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
// use tokio_util::codec::Encoder;
// use rustls;
use rustls::crypto::aws_lc_rs as provider;
// use rustls::client::ServerCertVerified;
use rustls::crypto::CryptoProvider;
use rustls::RootCertStore;
// use rustls::{Certificate, OwnedTrustAnchor, PrivateKey, RootCertStore, ServerName};
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls_pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use tokio_rustls::{
    // webpki,
    TlsConnector,
};
// use x509_parser::parse_x509_certificate;
// use x509_parser::prelude::X509Certificate;

type AFnResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

/*
pub fn load_certs<P>(path: P) -> std::io::Result<Vec<Certificate>>
where
    P: AsRef<Path>,
{
    certs(&mut BufReader::new(File::open(path.as_ref())?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid certificate"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

pub fn load_keys<P>(path: P) -> std::io::Result<Vec<PrivateKey>>
where
    P: AsRef<Path>,
{
    // println!("{:?}", rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?)));
    rsa_private_keys(&mut BufReader::new(File::open(path.as_ref())?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}
*/

pub fn load_certs<P>(path: P) -> std::io::Result<Vec<CertificateDer<'static>>>
where
    P: AsRef<Path>,
{
    let certfile = File::open(path).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    certs(&mut reader).collect()
}

pub fn load_keys<P>(path: P) -> std::io::Result<Vec<PrivateKeyDer<'static>>>
where
    P: AsRef<Path>,
{
    // TODO: support other keys
    pkcs8_private_keys(&mut BufReader::new(File::open(path.as_ref())?))
        .map(|k_| k_.and_then(|k| Ok(PrivateKeyDer::Pkcs8(k))))
        .collect()
}

// client verifier
/*
struct AcceptAllVerifier {
    certs: Vec<rustls::Certificate>,
}

impl rustls::client::ServerCertVerifier for AcceptAllVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &Certificate,
        intermediates: &[Certificate],
        server_name: &ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Always return yes - WARNING: UNSAFE !

        println!("Verify!!");
        // println!("end_entity: {:?}", end_entity);
        // println!("server_name: {:?}", server_name);
        Ok(rustls::client::ServerCertVerified::assertion())
    }

    /*
    fn verify_server_cert(
        &self
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
    */
}

struct SelfSignedVerifier {
    certs: Vec<rustls::Certificate>,
}

impl rustls::client::ServerCertVerifier for SelfSignedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &Certificate,
        intermediates: &[Certificate],
        server_name: &ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let x509_cert: X509Certificate = parse_x509_certificate(&end_entity.0)
            .map_err(|_| rustls::Error::InvalidCertificateSignature)?
            .1;
        x509_cert
            .verify_signature(None)
            .map(|_| rustls::client::ServerCertVerified::assertion())
            .map_err(|_| rustls::Error::InvalidCertificateSignature)
    }
}
*/
// End client verifier

#[tokio::main]
async fn main() -> AFnResult<()> {
    let arg: Vec<String> = std::env::args().skip(1).take(5).collect();

    // let empty_str = String::new();
    let panic_msg =
        "Please run: cargo run -- ip:port root_ca.pem mydomain2.org client1.crt client1.key";
    let (addr, root_ca_path, domain_arg, client_cert, client_key) = match arg.len() {
        5 => (&arg[0], &arg[1], arg[2].clone(), &arg[3], &arg[4]),
        _ => panic!("{}", panic_msg),
    };

    let mut root_store = RootCertStore::empty();
    // Add CA
    let mut pem = BufReader::new(File::open(root_ca_path)?);
    let certs = certs(&mut pem).map(|c| c.unwrap());
    root_store.add_parsable_certificates(certs);

    // client cert & key
    let client_certs = load_certs(client_cert).unwrap();
    let mut client_keys = load_keys(client_key).unwrap();

    let suites = provider::DEFAULT_CIPHER_SUITES.to_vec();
    let versions = rustls::DEFAULT_VERSIONS.to_vec();
    let config = rustls::ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: suites,
            ..provider::default_provider()
        }
        .into(),
    )
    .with_protocol_versions(&versions)
    .expect("inconsistent cipher-suite/versions selected")
    .with_root_certificates(root_store)
    .with_client_auth_cert(
        client_certs,
        client_keys
            .pop()
            .ok_or("Unable to read at least one client key")
            .unwrap(),
    )
    .unwrap();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr[..]).await?;

    // Note: this should be set to your value in cert / subjectAltName
    let domain = ServerName::try_from(domain_arg)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid domain"))?;
    let stream = connector.connect(domain, stream).await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let to_send = b"foo client authHHhH";
    writer.write_all(to_send).await?;

    let buffer_len = to_send.len();
    let mut buffer: Vec<u8> = vec![0; buffer_len];
    // let mut buffer = String::new();
    let n = reader.read(&mut buffer[..]).await?;

    let s = std::str::from_utf8(&buffer[0..n]);
    println!(
        "Sent: {:?} - Received: {:?}",
        std::str::from_utf8(to_send),
        s
    );
    Ok(())
}
