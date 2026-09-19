use std::fs;
use std::path::Path;

pub struct MachOScannerA4;

impl MachOScannerA4 {

    pub fn scan_binary(app_path: &Path) -> std::io::Result<MachOInfo> {
        let binary = Self::find_main_binary(app_path)?;

        let data = fs::read(&binary)?;

        let is_macho = Self::check_magic(&data);
        let has_signature = Self::detect_signature_section(&data);

        Ok(MachOInfo {
            path: binary.to_string_lossy().to_string(),
            is_macho,
            has_signature_slot: has_signature,
            size: data.len(),
        })
    }

    fn find_main_binary(app_path: &Path) -> std::io::Result<std::path::PathBuf> {
        let contents = fs::read_dir(app_path)?;

        for entry in contents {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap() == "app" {
                let app_contents = fs::read_dir(path)?;

                for file in app_contents {
                    let file = file?;
                    let fpath = file.path();

                    if fpath.extension().is_none() {
                        continue;
                    }

                    if fpath.extension().unwrap() == "dylib"
                        || fpath.file_name().unwrap().to_string_lossy().len() > 0
                    {
                        // fallback: main binary is usually same name as folder
                        return Ok(fpath);
                    }
                }
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Mach-O binary not found",
        ))
    }

    fn check_magic(data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }

        // Mach-O magic numbers (simplified)
        let magic = &data[0..4];

        matches!(
            magic,
            [0xfe, 0xed, 0xfa, 0xce] | // 32-bit
            [0xce, 0xfa, 0xed, 0xfe] | // reversed
            [0xfe, 0xed, 0xfa, 0xcf] | // 64-bit
            [0xcf, 0xfa, 0xed, 0xfe]
        )
    }

    fn detect_signature_section(data: &[u8]) -> bool {
        // simplified heuristic:
        // look for LC_CODE_SIGNATURE marker (not full parse yet)

        let pattern = b"LC_CODE_SIGNATURE";

        data.windows(pattern.len()).any(|w| w == pattern)
    }
}

pub struct MachOInfo {
    pub path: String,
    pub is_macho: bool,
    pub has_signature_slot: bool,
    pub size: usize,
}