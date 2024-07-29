use std::env;
use std::error::Error;
use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::aws_lc_rs as provider;
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::{DigitallySignedStruct, RootCertStore};
use rustls_pki_types::{CertificateDer, ServerName, UnixTime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

type AFnResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// client verifier

#[derive(Debug)]
pub struct NoCertificateVerification(CryptoProvider);

impl NoCertificateVerification {
    pub fn new(provider: CryptoProvider) -> Self {
        Self(provider)
    }
}

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
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
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

// End client verifier

#[tokio::main]
async fn main() -> AFnResult<()> {
    let arg: Vec<String> = env::args().skip(1).take(1).collect();

    // let empty_str = String::new();
    let addr = match arg.len() {
        1 => &arg[0],
        _ => panic!("Please run: cargo run -- ip:port"),
    };

    let root_store = RootCertStore::empty();
    let suites = provider::DEFAULT_CIPHER_SUITES.to_vec();
    let versions = rustls::DEFAULT_VERSIONS.to_vec();

    let mut config = rustls::ClientConfig::builder_with_provider(
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

    config
        .dangerous()
        .set_certificate_verifier(Arc::new(NoCertificateVerification::new(
            provider::default_provider(),
        )));

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr[..]).await?;

    let domain = ServerName::try_from("invalid1")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid domain"))?;
    let stream = connector.connect(domain, stream).await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let to_send = b"foo bar BAz";
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
