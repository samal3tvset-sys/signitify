use libc::c_char;
use std::ffi::CString;
use std::ptr;

// ===============================
// FFI bindings (libimobiledevice)
// ===============================

#[repr(C)]
pub struct idevice_t;

#[repr(C)]
pub struct lockdownd_client_t;

extern "C" {
    fn idevice_new(device: *mut *mut idevice_t, udid: *const c_char) -> i32;
    fn idevice_free(device: *mut idevice_t);

    fn lockdownd_client_new_with_handshake(
        device: *mut idevice_t,
        client: *mut *mut lockdownd_client_t,
        label: *const c_char,
    ) -> i32;

    fn lockdownd_client_free(client: *mut lockdownd_client_t);

    fn lockdownd_start_service(
        client: *mut lockdownd_client_t,
        identifier: *const c_char,
        port: *mut u16,
    ) -> i32;
}

// ===============================
// MAIN DEVICE SESSION
// ===============================

pub struct DeviceSessionV2 {
    device: *mut idevice_t,
    client: *mut lockdownd_client_t,
}

impl DeviceSessionV2 {

    // -------------------------------
    // CONNECT TO DEVICE (lockdown handshake)
    // -------------------------------
    pub fn connect(udid: &str) -> Option<Self> {
        unsafe {
            let c_udid = CString::new(udid).ok()?;
            let mut device: *mut idevice_t = ptr::null_mut();

            if idevice_new(&mut device, c_udid.as_ptr()) != 0 {
                return None;
            }

            let mut client: *mut lockdownd_client_t = ptr::null_mut();
            let label = CString::new("signitify-v2").ok()?;

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

    // -------------------------------
    // OPEN SERVICE (installation_proxy, AFC, etc.)
    // -------------------------------
    pub fn start_service(&self, service: &str) -> Option<u16> {
        unsafe {
            let c_service = CString::new(service).ok()?;
            let mut port: u16 = 0;

            let res = lockdownd_start_service(
                self.client,
                c_service.as_ptr(),
                &mut port
            );

            if res != 0 {
                return None;
            }

            Some(port)
        }
    }

    // -------------------------------
    // SIMPLE HELPER: INSTALLATION PROXY PORT
    // -------------------------------
    pub fn open_installation_proxy(&self) -> Option<u16> {
        self.start_service("com.apple.mobile.installation_proxy")
    }

    // -------------------------------
    // SIMPLE HELPER: AFC (file transfer)
    // -------------------------------
    pub fn open_afc(&self) -> Option<u16> {
        self.start_service("com.apple.afc")
    }
}

// ===============================
// CLEANUP (Drop trait)
// ===============================

impl Drop for DeviceSessionV2 {
    fn drop(&mut self) {
        unsafe {
            if !self.client.is_null() {
                lockdownd_client_free(self.client);
            }
            if !self.device.is_null() {
                idevice_free(self.device);
            }
        }
    }
}