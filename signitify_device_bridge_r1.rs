use std::process::Command;

pub struct DeviceBridgeR1;

impl DeviceBridgeR1 {

    // 1. list connected devices
    pub fn list_devices() -> Vec<String> {
        let output = Command::new("idevice_id")
            .arg("-l")
            .output();

        if let Ok(o) = output {
            let text = String::from_utf8_lossy(&o.stdout);
            return text.lines().map(|s| s.to_string()).collect();
        }

        vec![]
    }

    // 2. get device info (basic lockdownd call)
    pub fn device_info(udid: &str) -> Option<String> {
        let output = Command::new("ideviceinfo")
            .arg("-u")
            .arg(udid)
            .output()
            .ok()?;

        Some(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // 3. trust pairing check
    pub fn is_paired(udid: &str) -> bool {
        let output = Command::new("idevicepair")
            .arg("validate")
            .arg("-u")
            .arg(udid)
            .output();

        if let Ok(o) = output {
            return o.status.success();
        }

        false
    }
}