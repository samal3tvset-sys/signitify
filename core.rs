pub struct SignitifyCore {
    pub devices: Vec<String>,
}

impl SignitifyCore {
    pub fn new() -> Self {
        Self {
            devices: vec![],
        }
    }

    pub fn add_device(&mut self, udid: String) {
        self.devices.push(udid);
    }
}