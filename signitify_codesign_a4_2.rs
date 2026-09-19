use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

pub struct CodeSignEngineA4;

impl CodeSignEngineA4 {

    /// Главная функция подготовки "подписи"
    pub fn prepare_signature_blob(binary_path: &Path) -> std::io::Result<CodeSignatureA4> {

        let data = fs::read(binary_path)?;

        let hash = Self::hash_binary(&data);
        let sections = Self::simulate_sections(&data);

        Ok(CodeSignatureA4 {
            binary_hash: hash,
            section_count: sections.len(),
            signature_blob: Self::build_fake_signature(&hash, sections.len()),
        })
    }

    fn hash_binary(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);

        let result = hasher.finalize();

        hex::encode(result)
    }

    fn simulate_sections(data: &[u8]) -> Vec<SectionA4> {
        // упрощённая модель Mach-O секций
        let chunk_size = 4096;
        let mut sections = Vec::new();

        let mut i = 0;
        while i < data.len() {
            let end = std::cmp::min(i + chunk_size, data.len());

            let chunk = &data[i..end];

            let mut hasher = Sha256::new();
            hasher.update(chunk);

            sections.push(SectionA4 {
                index: i / chunk_size,
                hash: hex::encode(hasher.finalize()),
                size: chunk.len(),
            });

            i += chunk_size;
        }

        sections
    }

    fn build_fake_signature(hash: &str, sections: usize) -> String {
        format!(
            "SIGNITIFY-A4-CODESIGN\nBINARY_HASH={}\nSECTIONS={}\nSTATUS=PREPARED\n",
            hash,
            sections
        )
    }
}

pub struct CodeSignatureA4 {
    pub binary_hash: String,
    pub section_count: usize,
    pub signature_blob: String,
}

pub struct SectionA4 {
    pub index: usize,
    pub hash: String,
    pub size: usize,
}