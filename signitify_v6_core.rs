use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::signitify_bridge_v5::SessionManagerV5;

// ===============================
// DEVICE PROFILE
// ===============================

pub struct DeviceProfile {
    pub udid: String,
    pub name: String,
    pub trusted: bool,
}

// ===============================
// SIGNITIFY CORE V6
// ===============================

pub struct SignitifyCoreV6 {
    pub devices: HashMap<String, DeviceProfile>,
    pub sessions: HashMap<String, SessionManagerV5>,
}

impl SignitifyCoreV6 {

    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    // ===============================
    // ADD DEVICE
    // ===============================
    pub fn add_device(&mut self, udid: &str, name: &str) {
        self.devices.insert(udid.to_string(), DeviceProfile {
            udid: udid.to_string(),
            name: name.to_string(),
            trusted: false,
        });
    }

    // ===============================
    // TRUST DEVICE (pairing cache)
    // ===============================
    pub fn trust_device(&mut self, udid: &str) {
        if let Some(d) = self.devices.get_mut(udid) {
            d.trusted = true;
            println!("[V6] device trusted: {}", d.name);
        }
    }

    // ===============================
    // CONNECT DEVICE SESSION
    // ===============================
    pub fn connect_device(&mut self, udid: &str) -> bool {

        if let Some(dev) = self.devices.get(udid) {

            if !dev.trusted {
                println!("[V6] device not trusted");
                return false;
            }

            let mut session = SessionManagerV5::new(udid);

            if session.connect() {
                self.sessions.insert(udid.to_string(), session);
                println!("[V6] connected: {}", dev.name);
                return true;
            }
        }

        false
    }

    // ===============================
    // INSTALL IPA TO DEVICE
    // ===============================
    pub fn install_to_device(&mut self, udid: &str, name: &str, ipa: &str) {

        if let Some(session) = self.sessions.get_mut(udid) {
            session.add_app(name, ipa);
            println!("[V6] app added to device {}", udid);
        }
    }

    // ===============================
    // DAEMON LOOP (CORE ENGINE)
    // ===============================
    pub fn run_daemon(&mut self) {

        loop {
            println!("[V6] global tick...");

            for (_, session) in self.sessions.iter_mut() {
                session.run_daemon_tick();
            }

            thread::sleep(Duration::from_secs(60 * 60 * 6));
        }
    }
}