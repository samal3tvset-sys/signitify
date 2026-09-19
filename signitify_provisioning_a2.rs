use std::fs;

pub struct ProvisioningA2;

impl ProvisioningA2 {
    pub fn extract_minimal_info(path: &str) -> Option<(String, String)> {
        let data = fs::read(path).ok()?;

        let text = String::from_utf8_lossy(&data);

        let team_marker = "TeamIdentifier";
        let app_marker = "application-identifier";

        let team = text
            .split(team_marker)
            .nth(1)?
            .chars()
            .take(10)
            .collect::<String>();

        let app = text
            .split(app_marker)
            .nth(1)?
            .chars()
            .take(40)
            .collect::<String>();

        Some((team, app))
    }
}