use std::process::Command;

pub struct DeviceBridgeA1;

impl DeviceBridgeA1 {
    pub fn list_ios_devices() -> Vec<String> {
        let output = Command::new("idevice_id")
            .arg("-l")
            .output()
            .ok()?;

        let text = String::from_utf8_lossy(&output.stdout);

        text.lines().map(|s| s.to_string()).collect()
    }

    pub fn push_app_to_device(ipa_path: &str) -> bool {
        Command::new("ideviceinstaller")
            .arg("-i")
            .arg(ipa_path)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}