use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::signitify_device_bridge_a1::DeviceBridgeA1;
use crate::signitify_realsign_a6::RealSignEngineA6;

pub struct SignitifyDaemonF1;

impl SignitifyDaemonF1 {

    pub fn start(self) {
        println!("Signitify Daemon F1 started...");

        loop {
            self.tick();

            // check every 6 hours
            thread::sleep(Duration::from_secs(60 * 60 * 6));
        }
    }

    fn tick(&self) {
        println!("Daemon tick: checking signed apps...");

        let apps = self.load_installed_apps();

        for app in apps {
            if self.is_expiring_soon(&app) {
                println!("Re-signing app: {}", app.name);

                self.resign_and_install(&app);
            }
        }
    }

    fn load_installed_apps(&self) -> Vec<TrackedApp> {
        vec![
            TrackedApp {
                name: "SignitifyDemo".to_string(),
                ipa_path: "demo.ipa".to_string(),
                days_left: 3,
            }
        ]
    }

    fn is_expiring_soon(&self, app: &TrackedApp) -> bool {
        app.days_left <= 3
    }

    fn resign_and_install(&self, app: &TrackedApp) {
        println!("Rebuilding IPA: {}", app.name);

        let identity = crate::signitify_identity_a5::AppleIdentityA5::new(
            "DEMO_TEAM",
            "com.signitify.demo"
        );

        let _signed = RealSignEngineA6::build_signed_package(
            std::path::Path::new(&app.ipa_path),
            &identity
        );

        println!("Re-sign complete for {}", app.name);

        let ok = DeviceBridgeA1::push_app_to_device(&app.ipa_path);

        if ok {
            println!("Installed on device: {}", app.name);
        } else {
            println!("Install failed: {}", app.name);
        }
    }
}

pub struct TrackedApp {
    pub name: String,
    pub ipa_path: String,
    pub days_left: i32,
}