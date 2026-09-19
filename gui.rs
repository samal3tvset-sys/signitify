use eframe::egui;
use std::sync::mpsc::{channel, Sender};

use crate::signitify_v6_core::SignitifyCoreV6;

pub struct SignitifyGUI {
    core: SignitifyCoreV6,

    selected_udid: String,
    dropped_file: Option<String>,

    log: Vec<String>,
    tx: Sender<String>,
}

impl SignitifyGUI {
    pub fn new(core: SignitifyCoreV6) -> Self {
        let (tx, rx) = channel::<String>();

        let mut gui = Self {
            core,
            selected_udid: String::new(),
            dropped_file: None,
            log: vec![],
            tx,
        };

        // log listener thread
        std::thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                println!("[GUI LOG] {}", msg);
            }
        });

        gui
    }

    // ===============================
    // RUN APP
    // ===============================
    pub fn run(self) -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(700.0, 500.0)),
            ..Default::default()
        };

        eframe::run_native(
            "Signitify V7",
            options,
            Box::new(|_cc| Box::new(self)),
        )
    }
}

// ===============================
// EGUi APP
// ===============================

impl eframe::App for SignitifyGUI {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("🚀 Signitify V7 Device Manager");

            ui.separator();

            // =========================
            // DEVICE LIST
            // =========================
            ui.label("Devices:");

            for (udid, dev) in &self.core.devices {
                if ui.selectable_label(self.selected_udid == *udid, &dev.name).clicked() {
                    self.selected_udid = udid.clone();
                }
            }

            ui.separator();

            // =========================
            // DRAG & DROP IPA
            // =========================

            ui.label("Drop IPA here:");

            let drop_zone = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

            if drop_zone.hovered() {
                ui.label("⬇ Drop file now");
            }

            // egui file drop
            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                for file in &ctx.input(|i| i.raw.dropped_files.clone()) {
                    if let Some(path) = &file.path {
                        self.dropped_file = Some(path.to_string_lossy().to_string());
                        self.tx.send(format!("IPA dropped: {}", path.to_string_lossy())).ok();
                    }
                }
            }

            if let Some(file) = &self.dropped_file {
                ui.label(format!("Selected IPA: {}", file));
            }

            ui.separator();

            // =========================
            // ACTION BUTTONS
            // =========================

            if ui.button("➕ Add Device").clicked() {
                self.tx.send("Add device clicked".into()).ok();
            }

            if ui.button("🔐 Trust Device").clicked() {
                self.core.trust_device(&self.selected_udid);
                self.tx.send("Device trusted".into()).ok();
            }

            if ui.button("🔌 Connect Device").clicked() {
                let ok = self.core.connect_device(&self.selected_udid);
                self.tx.send(format!("Connect: {}", ok)).ok();
            }

            if ui.button("📲 Install IPA").clicked() {

                if let Some(ipa) = &self.dropped_file {
                    self.core.install_to_device(
                        &self.selected_udid,
                        "DroppedApp",
                        ipa
                    );

                    self.tx.send("Install triggered".into()).ok();
                }
            }

            if ui.button("🔄 Start Daemon").clicked() {
                self.tx.send("Daemon started".into()).ok();

                let mut core = self.core.clone();
                std::thread::spawn(move || {
                    core.run_daemon();
                });
            }

            ui.separator();

            // =========================
            // LOG AREA
            // =========================
            ui.label("Logs:");

            for msg in &self.log {
                ui.label(msg);
            }
        });
    }
}