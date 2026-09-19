use libc::c_char;
use std::ffi::CString;
use std::ptr;
use std::io::{Read, Write};
use std::net::TcpStream;

// ===============================
// LOW LEVEL FFI (lockdown only)
// ===============================

#[repr(C)]
pub struct idevice_t;

#[repr(C)]
pub struct lockdownd_client_t;

extern "C" {
    fn idevice_new(device: *mut *mut idevice_t, udid: *const c_char) -> i32;

    fn lockdownd_client_new_with_handshake(
        device: *mut idevice_t,
        client: *mut *mut lockdownd_client_t,
        label: *const c_char,
    ) -> i32;

    fn lockdownd_start_service(
        client: *mut lockdownd_client_t,
        identifier: *const c_char,
        port: *mut u16,
    ) -> i32;
}

// ===============================
// DEVICE SESSION V4
// ===============================

pub struct DeviceSessionV4 {
    device: *mut idevice_t,
    client: *mut lockdownd_client_t,
}

impl DeviceSessionV4 {

    pub fn connect(udid: &str) -> Option<Self> {
        unsafe {
            let c_udid = CString::new(udid).ok()?;
            let mut device: *mut idevice_t = ptr::null_mut();

            if idevice_new(&mut device, c_udid.as_ptr()) != 0 {
                return None;
            }

            let mut client: *mut lockdownd_client_t = ptr::null_mut();
            let label = CString::new("signitify-v4").ok()?;

            if lockdownd_client_new_with_handshake(
                device,
                &mut client,
                label.as_ptr()
            ) != 0 {
                return None;
            }

            Some(Self { device, client })
        }
    }

    // ===============================
    // RAW SERVICE SOCKET OPENING
    // ===============================
    fn open_service_socket(&self, service: &str) -> Option<TcpStream> {
        unsafe {
            let c_service = CString::new(service).ok()?;
            let mut port: u16 = 0;

            let res = lockdownd_start_service(
                self.client,
                c_service.as_ptr(),
                &mut port
            );

            if res != 0 || port == 0 {
                return None;
            }

            // localhost tunnel created by usbmuxd
            let addr = format!("127.0.0.1:{}", port);

            TcpStream::connect(addr).ok()
        }
    }

    // ===============================
    // AFC PROTOCOL (RAW STREAM)
    // ===============================
    pub fn afc_write_file(&self, remote_path: &str, data: &[u8]) -> bool {

        let mut stream = match self.open_service_socket("com.apple.afc") {
            Some(s) => s,
            None => return false,
        };

        println!("[AFC-V4] writing to {}", remote_path);

        // ⚠️ simplified AFC protocol frame
        let request = self.build_afc_write_packet(remote_path, data);

        if stream.write_all(&request).is_err() {
            return false;
        }

        let mut response = vec![0u8; 4096];
        let _ = stream.read(&mut response);

        println!("[AFC-V4] response received");

        true
    }

    // ===============================
    // INSTALLATION PROXY (PLIST RPC)
    // ===============================
    pub fn install_application(&self, bundle_path: &str) -> bool {

        let mut stream = match self.open_service_socket("com.apple.mobile.installation_proxy") {
            Some(s) => s,
            None => return false,
        };

        println!("[INSTALL-V4] sending plist request...");

        let plist = self.build_install_plist(bundle_path);

        if stream.write_all(plist.as_bytes()).is_err() {
            return false;
        }

        let mut resp = vec![0u8; 8192];
        let _ = stream.read(&mut resp);

        println!("[INSTALL-V4] response received");
        true
    }

    // ===============================
    // PLIST BUILDER (INSTALL REQUEST)
    // ===============================
    fn build_install_plist(&self, path: &str) -> String {
        format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
    <key>Command</key>
    <string>InstallApplication</string>

    <key>PackagePath</key>
    <string>{}</string>

    <key>ClientOptions</key>
    <dict>
        <key>PackageType</key>
        <string>Developer</string>
    </dict>
</dict>
</plist>
"#, path)
    }

    // ===============================
    // AFC PACKET (SIMPLIFIED MODEL)
    // ===============================
    fn build_afc_write_packet(&self, path: &str, data: &[u8]) -> Vec<u8> {
        let mut packet = Vec::new();

        packet.extend_from_slice(b"AFC_WRITE_SIM\n");
        packet.extend_from_slice(path.as_bytes());
        packet.extend_from_slice(b"\n");
        packet.extend_from_slice(&(data.len() as u32).to_le_bytes());
        packet.extend_from_slice(data);

        packet
    }
}

// ===============================
// CLEANUP
// ===============================

impl Drop for DeviceSessionV4 {
    fn drop(&mut self) {
        unsafe {
            // cleanup handled by libimobiledevice
        }
    }
}