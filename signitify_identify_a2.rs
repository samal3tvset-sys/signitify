pub struct SignitifyIdentityA2 {
    pub team_id: String,
    pub bundle_id: String,
}

impl SignitifyIdentityA2 {
    pub fn new(team_id: &str, bundle_id: &str) -> Self {
        Self {
            team_id: team_id.to_string(),
            bundle_id: bundle_id.to_string(),
        }
    }

    pub fn apple_style_app_id(&self) -> String {
        format!("{}.{}", self.team_id, self.bundle_id)
    }
}