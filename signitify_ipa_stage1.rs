use std::fs;
use std::path::Path;

pub struct IPAStageA1;

impl IPAStageA1 {
    pub fn mark_app(&self, extract_path: &Path) -> std::io::Result<()> {
        let marker = extract_path.join("Payload/_SIGNITIFY_STAGE1");

        fs::write(
            marker,
            "stage=1\nstatus=prepared_for_signing"
        )?;

        Ok(())
    }
}