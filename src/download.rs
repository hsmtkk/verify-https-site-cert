use anyhow::{bail, Result};
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::vec::Vec;

pub trait CertDownloader {
    fn download_cert(&self, fqdn:&str) -> Result<Vec<u8>>;
}

pub fn new() -> impl CertDownloader {
    CertDownloaderImpl::new()
}

struct CertDownloaderImpl {}

impl CertDownloaderImpl {
    fn new() -> CertDownloaderImpl {
        CertDownloaderImpl{}
    }
}

impl CertDownloader for CertDownloaderImpl {
    fn download_cert(&self, fqdn:&str) -> Result<Vec<u8>>{
        let connector = TlsConnector::new()?;
        let stream = TcpStream::connect(format!("{}:443", fqdn))?;
        let tls_stream = connector.connect(fqdn, stream)?;
        let cert = match tls_stream.peer_certificate() {
            Ok(opt_cert) => {
                match opt_cert {
                    Some(cert) => cert,
                    None => bail!("failed to get peer cert"),
                }
            },
            Err(e) => bail!("failed to get peer cert; {}", e),
        };
        let der = cert.to_der()?;
        Ok(der)
    }
}