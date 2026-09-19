pub struct MachOLayoutA4;

impl MachOLayoutA4 {

    pub fn build_layout(binary_size: usize) -> MachOLayoutInfo {
        let signature_offset = binary_size;
        let signature_size = Self::estimate_signature_size(binary_size);

        MachOLayoutInfo {
            binary_size,
            signature_offset,
            signature_size,
            total_size: binary_size + signature_size,
        }
    }

    fn estimate_signature_size(size: usize) -> usize {
        // упрощённая модель Apple signature growth
        (size / 10) + 4096
    }

    pub fn generate_lc_code_signature(&self, layout: &MachOLayoutInfo) -> LCCodeSignatureA4 {
        LCCodeSignatureA4 {
            cmd: "LC_CODE_SIGNATURE".to_string(),
            data_offset: layout.signature_offset,
            data_size: layout.signature_size,
        }
    }
}

pub struct MachOLayoutInfo {
    pub binary_size: usize,
    pub signature_offset: usize,
    pub signature_size: usize,
    pub total_size: usize,
}

pub struct LCCodeSignatureA4 {
    pub cmd: String,
    pub data_offset: usize,
    pub data_size: usize,
}