use std::fs;
use std::path::{Path, PathBuf};

pub struct IPAInjectorA3;

impl IPAInjectorA3 {

    pub fn inject_all(
        extract_path: &Path,
        entitlements: &str,
        provisioning_path: &str,
    ) -> std::io::Result<()> {

        let payload = extract_path.join("Payload");

        self.inject_entitlements(&payload, entitlements)?;
        self.inject_provisioning(&payload, provisioning_path)?;

        Ok(())
    }

    fn inject_entitlements(
        &self,
        payload: &Path,
        entitlements: &str
    ) -> std::io::Result<()> {

        let ent_path = payload.join("_entitlements.plist");

        fs::write(ent_path, entitlements)?;

        Ok(())
    }

    fn inject_provisioning(
        &self,
        payload: &Path,
        provisioning_path: &str
    ) -> std::io::Result<()> {

        let app_dirs = fs::read_dir(payload)?;

        for entry in app_dirs {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap() == "app" {

                let embedded_path = path.join("embedded.mobileprovision");

                let data = fs::read(provisioning_path)?;

                fs::write(embedded_path, data)?;
            }
        }

        Ok(())
    }
}