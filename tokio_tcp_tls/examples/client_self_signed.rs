use std::env;
use std::error::Error;
use std::sync::Arc;
// use std::time::SystemTime;

// use std::fs::File;
// use std::io::BufReader;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
// use tokio_util::codec::Encoder;
use rustls;
// use rustls::client::ServerCertVerified;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
// use rustls::client::WebPkiServerVerifier;
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
// use rustls::server::WebPkiClientVerifier;
use rustls::{ClientConfig, DigitallySignedStruct, RootCertStore, SignatureScheme};
// use rustls_pemfile::{read_all, read_one, Item};
use rustls_pki_types::{CertificateDer, ServerName, UnixTime};
// use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::TlsConnector;
// use x509_parser::parse_x509_certificate;
// use x509_parser::prelude::X509Certificate;
// use bytes::Buf;

// use rustls_pki_types::{CertificateDer, UnixTime};
// use rustls::client::danger::HandshakeSignatureValid;
// use rustls::client::WebPkiServerVerifier;
// use rustls::DigitallySignedStruct;

type AFnResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// client verifier

#[derive(Debug)]
struct AcceptAllVerifier {
    // certs: Vec<Certificate>,
}

impl ServerCertVerifier for AcceptAllVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        // WebPkiServerVerifier::verify_tls12_signature(message, cert, dss)
        verify_tls12_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &rustls::crypto::ring::default_provider().signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }

    /*
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
    */

    /*
    fn verify_server_cert(
        &self
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
    */
}

/*
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
            .map_err(|e|
                // rustls::Error::InvalidCertificate
                // TODO: can we reuse X509Error ?
                rustls::CertificateError::BadEncoding)?
            .1;
        x509_cert.verify_signature(None)
        /*
        .map(|_| rustls::client::ServerCertVerified::assertion())
        .map_err(|_|
            rustls::Error::InvalidCertificate
        )
        */
    }
}
*/

// End client verifier

#[tokio::main]
async fn main() -> AFnResult<()> {
    // let addr_in: &str = "127.0.0.1:6139";

    let arg: Vec<String> = env::args().skip(1).take(1).collect();

    // let empty_str = String::new();
    let addr = match arg.len() {
        1 => &arg[0],                                                     // TODO: proper error
        _ => panic!("Please run: cargo run -- ip:port cert.pem key.pem"), // TODO: proper error
    };

    let verifier = Arc::new(AcceptAllVerifier {});
    // let verifier1 = Arc::new(SelfSignedVerifier { certs: vec![] });

    /*
    let config = ClientConfig::builder()
        // .with_safe_defaults()
        // .with_custom_certificate_verifier(verifier1)
        // .with_no_client_auth();
        .with_safe_defaults()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();
    */

    let mut root_store = RootCertStore::empty();
    let suites = rustls::crypto::ring::DEFAULT_CIPHER_SUITES.to_vec();
    let versions = rustls::DEFAULT_VERSIONS.to_vec();

    let config = rustls::ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: suites,
            ..rustls::crypto::ring::default_provider()
        }
        .into(),
    )
    .with_protocol_versions(&versions)
    .expect("inconsistent cipher-suite/versions selected")
    .with_root_certificates(root_store)
    .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr[..]).await?;

    let domain = ServerName::try_from("invalid1")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid domain"))?;
    let mut stream = connector.connect(domain, stream).await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let to_send = b"foo";
    writer.write_all(to_send).await?;

    let buffer_len = 3;
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
