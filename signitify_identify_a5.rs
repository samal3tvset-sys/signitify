pub struct AppleIdentityA5 {
    pub team_id: String,
    pub apple_id: String,
    pub certificate_fingerprint: String,
}

impl AppleIdentityA5 {

    pub fn new(team_id: &str, apple_id: &str) -> Self {
        Self {
            team_id: team_id.to_string(),
            apple_id: apple_id.to_string(),
            certificate_fingerprint: String::new(),
        }
    }

    pub fn bind_certificate(&mut self, cert_data: &str) {
        // в реальности тут SHA1/SHA256 сертификата
        self.certificate_fingerprint = format!("CERT-{}", Self::hash(cert_data));
    }

    fn hash(input: &str) -> String {
        use sha2::{Sha256, Digest};

        let mut h = Sha256::new();
        h.update(input.as_bytes());

        hex::encode(h.finalize())
    }

    pub fn identity_string(&self) -> String {
        format!(
            "APPLE_IDENTITY::TEAM={}::CERT={}::USER={}",
            self.team_id,
            self.certificate_fingerprint,
            self.apple_id
        )
    }
}