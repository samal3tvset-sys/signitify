use std::path::Path;

use crate::signitify_identity_a5::AppleIdentityA5;
use crate::signitify_macho_layout_a4_3::MachOLayoutA4;
use crate::signitify_codesign_a4_2::CodeSignEngineA4;

pub struct RealSignEngineA6;

impl RealSignEngineA6 {

    pub fn build_signed_package(
        binary_path: &Path,
        identity: &AppleIdentityA5
    ) -> SignedPackageA6 {

        // 1. analyze binary
        let layout = MachOLayoutA4::build_layout(
            std::fs::metadata(binary_path).unwrap().len() as usize
        );

        // 2. generate code signature model
        let sig = CodeSignEngineA4::prepare_signature_blob(binary_path)
            .expect("signature build failed");

        // 3. bind identity
        let identity_str = identity.identity_string();

        SignedPackageA6 {
            identity: identity_str,
            binary_size: layout.binary_size,
            signature_blob: sig.signature_blob,
            layout_offset: layout.signature_offset,
            valid: true,
        }
    }
}