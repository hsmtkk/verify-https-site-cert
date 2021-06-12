mod verify;
use crate::verify::CertVerifier;

fn main() {
    let fqdn = "www.example.com";
    let verifier = verify::new();
    let result = verifier.verify_site(fqdn).unwrap();
    }
