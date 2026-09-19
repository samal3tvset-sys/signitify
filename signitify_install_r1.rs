use std::process::Command;

pub struct InstallEngineR1;

impl InstallEngineR1 {

    // main IPA install pipeline (AltStore-style)
    pub fn install_ipa(ipa_path: &str, udid: &str) -> bool {

        // step 1: ensure device is paired
        let paired = Command::new("idevicepair")
            .arg("validate")
            .arg("-u")
            .arg(udid)
            .status();

        if paired.map(|s| !s.success()).unwrap_or(true) {
            println!("Device not paired");
            return false;
        }

        // step 2: install via installation_proxy
        let status = Command::new("ideviceinstaller")
            .arg("-u")
            .arg(udid)
            .arg("-i")
            .arg(ipa_path)
            .status();

        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }
}