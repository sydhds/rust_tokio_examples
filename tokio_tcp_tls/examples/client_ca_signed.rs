use std::error::Error;
use std::sync::Arc;
// use std::time::SystemTime;

use std::fs::File;
use std::io::BufReader;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
// use tokio_util::codec::Encoder;
// use rustls;
use rustls::crypto::aws_lc_rs as provider;
use rustls::crypto::CryptoProvider;
use rustls::RootCertStore;
use tokio_rustls::{
    TlsConnector,
    // webpki
};
// use rustls::{Certificate, OwnedTrustAnchor, ServerName};
// use rustls::client::ServerCertVerified;
// use rustls_pemfile::{read_all, read_one, Item};
use rustls_pki_types::{
    // CertificateDer,
    ServerName,
};
// use x509_parser::parse_x509_certificate;
// use x509_parser::prelude::X509Certificate;

type AFnResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// client verifier

/*
struct AcceptAllVerifier {
    certs: Vec<rustls::Certificate>,
}

impl rustls::client::ServerCertVerifier for AcceptAllVerifier {
    fn verify_server_cert(&self,
                          end_entity: &Certificate,
                          intermediates: &[Certificate],
                          server_name: &ServerName,
                          scts: &mut dyn Iterator<Item=&[u8]>,
                          ocsp_response: &[u8],
                          now: SystemTime) -> Result<ServerCertVerified, rustls::Error> {
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
    fn verify_server_cert(&self,
                          end_entity: &Certificate,
                          intermediates: &[Certificate],
                          server_name: &ServerName,
                          scts: &mut dyn Iterator<Item=&[u8]>,
                          ocsp_response: &[u8],
                          now: SystemTime) -> Result<ServerCertVerified, rustls::Error> {

        let x509_cert: X509Certificate = parse_x509_certificate(&end_entity.0)
            .map_err(|_|
                rustls::Error::InvalidCertificateSignature
            )?
            .1;
        x509_cert.verify_signature(None)
            .map(|_| rustls::client::ServerCertVerified::assertion())
            .map_err(|_|
                rustls::Error::InvalidCertificateSignature
            )
    }
}
*/

// End client verifier

#[tokio::main]
async fn main() -> AFnResult<()> {
    let arg: Vec<String> = std::env::args().skip(1).take(3).collect();

    // let empty_str = String::new();
    let panic_msg = "Please run: cargo run -- ip:port certs/ca_signed/root_ca.pem mydomain.com";
    let (addr, root_ca_path, domain_arg) = match arg.len() {
        3 => (&arg[0], &arg[1], arg[2].clone()),
        _ => panic!("{}", panic_msg),
    };

    let mut root_store = RootCertStore::empty();

    // Add CA
    let mut pem = BufReader::new(File::open(root_ca_path)?);
    // let certs: Result<Vec<CertificateDer<'_>>, std::io::Error> =
    //     rustls_pemfile::certs(&mut pem).take(1).collect();
    // root_store.add(certs.unwrap().first().unwrap().clone()).unwrap();
    let certs = rustls_pemfile::certs(&mut pem).map(|c| c.unwrap());
    root_store.add_parsable_certificates(certs);

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
    .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr[..]).await?;

    // Note: this should be set to your value in cert / subjectAltName
    let domain = ServerName::try_from(domain_arg)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid domain"))?;
    let stream = connector.connect(domain, stream).await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let to_send = b"foo bazzzz zarb";
    writer.write_all(to_send).await?;

    let buffer_len = to_send.len();
    let mut buffer: Vec<u8> = vec![0; buffer_len];
    let n = reader.read(&mut buffer[..]).await?;

    let s = std::str::from_utf8(&buffer[0..n]);
    println!(
        "Sent: {:?} - Received: {:?}",
        std::str::from_utf8(to_send),
        s
    );
    Ok(())
}
