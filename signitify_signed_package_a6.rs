pub struct SignedPackageA6 {
    pub identity: String,
    pub binary_size: usize,
    pub signature_blob: String,
    pub layout_offset: usize,
    pub valid: bool,
}

impl SignedPackageA6 {

    pub fn export_manifest(&self) -> String {
        format!(r#"
SIGNITIFY SIGNED PACKAGE A6
----------------------------
IDENTITY: {}
BINARY_SIZE: {}
SIGNATURE_OFFSET: {}
VALID: {}
BLOB:
{}
"#,
            self.identity,
            self.binary_size,
            self.layout_offset,
            self.valid,
            self.signature_blob
        )
    }
}