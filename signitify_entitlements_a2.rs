use std::fs;

pub struct EntitlementsA2;

impl EntitlementsA2 {
    pub fn generate(team_id: &str, bundle_id: &str) -> String {
        let xml = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
    <key>application-identifier</key>
    <string>{}.{}</string>

    <key>get-task-allow</key>
    <true/>

    <key>keychain-access-groups</key>
    <array>
        <string>{}.{}</string>
    </array>

    <key>com.apple.developer.team-identifier</key>
    <string>{}</string>
</dict>
</plist>
"#, team_id, bundle_id, team_id, bundle_id, team_id);

        let _ = fs::write("signitify_entitlements_a2.plist", &xml);

        xml
    }
}