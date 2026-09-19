mod gui;
mod signitify_v6_core;

use signitify_v6_core::SignitifyCoreV6;
use gui::SignitifyGUI;

fn main() {
    let core = SignitifyCoreV6::new();
    let mut gui = SignitifyGUI::new(core);

    gui.run();
}