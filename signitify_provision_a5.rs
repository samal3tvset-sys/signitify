pub struct ProvisioningA5 {
    pub team_id: String,
    pub devices: Vec<String>,
    pub bundle_ids: Vec<String>,
}

impl ProvisioningA5 {

    pub fn new(team_id: &str) -> Self {
        Self {
            team_id: team_id.to_string(),
            devices: vec![],
            bundle_ids: vec![],
        }
    }

    pub fn add_device(&mut self, udid: &str) {
        self.devices.push(udid.to_string());
    }

    pub fn add_bundle(&mut self, bundle: &str) {
        self.bundle_ids.push(bundle.to_string());
    }

    pub fn is_device_allowed(&self, udid: &str) -> bool {
        self.devices.contains(&udid.to_string())
    }

    pub fn is_bundle_allowed(&self, bundle: &str) -> bool {
        self.bundle_ids.contains(&bundle.to_string())
    }
}