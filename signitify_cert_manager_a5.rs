use std::fs;

pub struct CertificateManagerA5;

impl CertificateManagerA5 {

    pub fn load_certificate(path: &str) -> std::io::Result<String> {
        let cert = fs::read_to_string(path)?;

        Ok(cert)
    }

    pub fn extract_fingerprint(cert: &str) -> String {
        use sha2::{Sha256, Digest};

        let mut h = Sha256::new();
        h.update(cert.as_bytes());

        hex::encode(h.finalize())
    }

    pub fn validate_certificate(cert: &str) -> bool {
        cert.contains("BEGIN CERTIFICATE")
    }
}