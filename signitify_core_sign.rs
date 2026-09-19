use std::fs;

pub struct SigCoreEngineA1 {
    pub team_tag: String,
    pub bundle_tag: String,
}

impl SigCoreEngineA1 {
    pub fn new(team_tag: &str, bundle_tag: &str) -> Self {
        Self {
            team_tag: team_tag.to_string(),
            bundle_tag: bundle_tag.to_string(),
        }
    }

    pub fn build_identity_string(&self) -> String {
        format!("{}.{}", self.team_tag, self.bundle_tag)
    }

    pub fn create_signature_stub(&self) -> String {
        let sig = format!(
            "SIGNITIFY-CORE-A1\nIDENTITY={}\nSTATUS=UNSIGNED_STUB",
            self.build_identity_string()
        );

        let _ = fs::write("signitify_signature_stub.txt", &sig);

        sig
    }
}