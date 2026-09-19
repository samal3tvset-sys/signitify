use std::io::{self, Write};

use crate::signitify_v6_core::SignitifyCoreV6;

pub struct SignitifyGUI {
    pub core: SignitifyCoreV6,
}

impl SignitifyGUI {

    pub fn new(core: SignitifyCoreV6) -> Self {
        Self { core }
    }

    pub fn run(&mut self) {
        loop {
            self.render();

            let choice = self.read_input();

            match choice.trim() {
                "1" => self.show_devices(),
                "2" => self.add_device(),
                "3" => self.trust_device(),
                "4" => self.connect_device(),
                "5" => self.install_app(),
                "6" => self.start_daemon(),
                "0" => {
                    println!("Exit...");
                    break;
                }
                _ => println!("Invalid option"),
            }
        }
    }

    fn render(&self) {
        println!("\n==============================");
        println!("       SIGNITIFY V6 GUI       ");
        println!("==============================");
        println!("1. List devices");
        println!("2. Add device");
        println!("3. Trust device");
        println!("4. Connect device");
        println!("5. Install IPA to device");
        println!("6. Start daemon");
        println!("0. Exit");
        println!("==============================");
        print!("Select: ");
        let _ = io::stdout().flush();
    }

    fn read_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input
    }

    // ----------------------------
    // UI ACTIONS
    // ----------------------------

    fn show_devices(&self) {
        println!("\n[DEVICES]");
        for (udid, dev) in &self.core.devices {
            println!("UDID: {} | Name: {} | Trusted: {}", udid, dev.name, dev.trusted);
        }
    }

    fn add_device(&mut self) {
        let udid = self.ask("Enter UDID:");
        let name = self.ask("Enter device name:");

        self.core.add_device(&udid, &name);
        println!("Device added");
    }

    fn trust_device(&mut self) {
        let udid = self.ask("Enter UDID to trust:");
        self.core.trust_device(&udid);
    }

    fn connect_device(&mut self) {
        let udid = self.ask("Enter UDID to connect:");

        if self.core.connect_device(&udid) {
            println!("Connected successfully");
        } else {
            println!("Connection failed");
        }
    }

    fn install_app(&mut self) {
        let udid = self.ask("Enter UDID:");
        let name = self.ask("App name:");
        let ipa = self.ask("IPA path:");

        self.core.install_to_device(&udid, &name, &ipa);
        println!("Install queued");
    }

    fn start_daemon(&mut self) {
        println!("Starting daemon (blocking loop)...");
        self.core.run_daemon();
    }

    fn ask(&self, msg: &str) -> String {
        print!("{} ", msg);
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }
}