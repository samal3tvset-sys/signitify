use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread;

use crate::signitify_device_bridge_v4::DeviceSessionV4;

// ===============================
// APP TRACKING MODEL
// ===============================

#[derive(Clone)]
pub struct TrackedAppV5 {
    pub name: String,
    pub ipa_path: String,
    pub install_time: u64,
    pub expiry_days: u64,
}

// ===============================
// SESSION MANAGER V5
// ===============================

pub struct SessionManagerV5 {
    pub device_udid: String,
    pub session: Option<DeviceSessionV4>,
    pub apps: Vec<TrackedAppV5>,
}

impl SessionManagerV5 {

    pub fn new(udid: &str) -> Self {
        Self {
            device_udid: udid.to_string(),
            session: None,
            apps: vec![],
        }
    }

    // ===============================
    // CONNECT + KEEP SESSION ALIVE
    // ===============================
    pub fn connect(&mut self) -> bool {
        match DeviceSessionV4::connect(&self.device_udid) {
            Some(s) => {
                self.session = Some(s);
                true
            }
            None => false,
        }
    }

    // ===============================
    // ADD APP TO TRACKER
    // ===============================
    pub fn add_app(&mut self, name: &str, ipa: &str) {
        let now = Self::now();

        self.apps.push(TrackedAppV5 {
            name: name.to_string(),
            ipa_path: ipa.to_string(),
            install_time: now,
            expiry_days: 7,
        });
    }

    // ===============================
    // MAIN LOOP (ALTSTORE STYLE DAEMON)
    // ===============================
    pub fn run_daemon(&mut self) {

        loop {
            println!("[V5] tick... checking apps");

            self.check_expiry_and_refresh();

            // every 6 hours (AltStore style behavior)
            thread::sleep(Duration::from_secs(60 * 60 * 6));
        }
    }

    // ===============================
    // EXPIRE CHECK + RE-SIGN TRIGGER
    // ===============================
    fn check_expiry_and_refresh(&mut self) {

        let now = Self::now();

        for app in &self.apps {

            let age = now - app.install_time;
            let days = age / 86400;

            if days >= app.expiry_days - 1 {
                println!("[V5] app expiring soon: {}", app.name);

                self.resign_and_reinstall(app);
            }
        }
    }

    // ===============================
    // RE-SIGN + RE-INSTALL PIPELINE
    // ===============================
    fn resign_and_reinstall(&self, app: &TrackedAppV5) {

        println!("[V5] re-signing {}", app.name);

        let session = match &self.session {
            Some(s) => s,
            None => return,
        };

        // STEP 1: re-install (pipeline reuse)
        let ok = session.install_application(&app.ipa_path);

        if ok {
            println!("[V5] refreshed app: {}", app.name);
        } else {
            println!("[V5] refresh failed: {}", app.name);
        }
    }

    // ===============================
    // TIME HELPERS
    // ===============================
    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}