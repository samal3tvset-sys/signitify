use crate::signitify_identity_a2::SignitifyIdentityA2;
use crate::signitify_entitlements_a2::EntitlementsA2;

pub struct SigningCoreA2;

impl SigningCoreA2 {
    pub fn prepare_signature_layer(team: &str, bundle: &str) -> String {
        let identity = SignitifyIdentityA2::new(team, bundle);

        let ent = EntitlementsA2::generate(team, bundle);

        format!(
            "SIGNITIFY SIGN LAYER A2\nIDENTITY={}\nENTITLEMENTS_READY\n{}",
            identity.apple_style_app_id(),
            ent
        )
    }
}