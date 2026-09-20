#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libimobiledevice/libimobiledevice.h>
#include <libimobiledevice/lockdown.h>
#include <libimobiledevice/installation_proxy.h>

// ===============================
// CONNECT TO DEVICE
// ===============================
idevice_t device = NULL;
lockdownd_client_t client = NULL;

int connect_device(const char *udid) {
    if (idevice_new(&device, udid) != IDEVICE_E_SUCCESS) {
        printf("[ERROR] Cannot connect to device\n");
        return -1;
    }

    if (lockdownd_client_new_with_handshake(device, &client, "signitify") != LOCKDOWN_E_SUCCESS) {
        printf("[ERROR] Lockdown handshake failed\n");
        return -1;
    }

    printf("[OK] Device connected\n");
    return 0;
}

// ===============================
// INSTALL IPA
// ===============================
int install_ipa(const char *ipa_path) {

    installation_proxy_client_t inst_proxy = NULL;
    lockdownd_service_descriptor_t service = NULL;

    // open installation_proxy service
    if (lockdownd_start_service(client,
        "com.apple.mobile.installation_proxy",
        &service) != LOCKDOWN_E_SUCCESS) {

        printf("[ERROR] Cannot start installation_proxy\n");
        return -1;
    }

    if (installation_proxy_client_new(device, service, &inst_proxy) != INSTPROXY_E_SUCCESS) {
        printf("[ERROR] Cannot create install proxy client\n");
        return -1;
    }

    printf("[OK] Installation proxy ready\n");

    // ===============================
    // PREPARE INSTALL REQUEST
    // ===============================
    plist_t options = plist_new_dict();
    plist_dict_insert_item(options, "PackageType", plist_new_string("Developer"));

    printf("[INFO] Installing IPA: %s\n", ipa_path);

    instproxy_error_t res = installation_proxy_install(inst_proxy, ipa_path, options, NULL, NULL);

    if (res != INSTPROXY_E_SUCCESS) {
        printf("[ERROR] Install failed: %d\n", res);
        return -1;
    }

    printf("[SUCCESS] IPA installed\n");

    plist_free(options);
    installation_proxy_client_free(inst_proxy);

    return 0;
}

// ===============================
// CLEANUP
// ===============================
void cleanup() {
    if (client) {
        lockdownd_client_free(client);
    }
    if (device) {
        idevice_free(device);
    }
}

// ===============================
// MAIN
// ===============================
int main(int argc, char *argv[]) {

    if (argc < 3) {
        printf("Usage: signitify_upload_ipa <UDID> <IPA_PATH>\n");
        return -1;
    }

    const char *udid = argv[1];
    const char *ipa_path = argv[2];

    if (connect_device(udid) != 0) {
        return -1;
    }

    if (install_ipa(ipa_path) != 0) {
        cleanup();
        return -1;
    }

    cleanup();
    return 0;
}