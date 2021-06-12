use anyhow::Result;
use rustls::{ClientConfig, ClientSession};
use std::sync::Arc;
use webpki::DNSNameRef;

pub trait CertVerifier {
    fn verify_site(&self, fqdn:&str) -> Result<()>;
}

pub fn new() -> impl CertVerifier {
    CertVerifierImpl::new()
}

struct CertVerifierImpl {}

impl CertVerifierImpl {
    fn new() -> CertVerifierImpl {
        CertVerifierImpl{}
    }
}

impl CertVerifier for CertVerifierImpl {
    fn verify_site(&self, fqdn:&str) -> Result<()>{
        let mut config = ClientConfig::new();
        config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        let rc_config = Arc::new(config);
        let dns_name = DNSNameRef::try_from_ascii_str(fqdn)?;
        let mut client = ClientSession::new(&rc_config, dns_name);
        Ok(())
    }
}