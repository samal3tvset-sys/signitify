#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include <libimobiledevice/libimobiledevice.h>
#include <libimobiledevice/lockdown.h>
#include <libimobiledevice/installation_proxy.h>
#include <plist/plist.h>

// ===============================
// GLOBAL DEVICE STATE
// ===============================

static idevice_t device = NULL;
static lockdownd_client_t client = NULL;

// ===============================
// CALLBACKS (FROM RUST)
// ===============================

typedef void (*log_callback)(const char *msg);

// ===============================
// INTERNAL LOG HELPER
// ===============================

static log_callback g_log = NULL;

static void log_msg(const char *msg) {
    if (g_log) {
        g_log(msg);
    } else {
        printf("%s\n", msg);
    }
}

// ===============================
// CONNECT DEVICE
// ===============================

int signitify_connect(const char *udid, log_callback cb) {
    g_log = cb;

    log_msg("[C-BRIDGE] connecting device...");

    if (idevice_new(&device, udid) != IDEVICE_E_SUCCESS) {
        log_msg("[C-BRIDGE] idevice_new failed");
        return -1;
    }

    if (lockdownd_client_new_with_handshake(device, &client, "signitify-v10") != LOCKDOWN_E_SUCCESS) {
        log_msg("[C-BRIDGE] lockdown handshake failed");
        return -1;
    }

    log_msg("[C-BRIDGE] device connected");
    return 0;
}

// ===============================
// INSTALL IPA (REAL PIPELINE)
// ===============================

int signitify_install_ipa(const char *ipa_path, log_callback cb) {
    g_log = cb;

    if (!device || !client) {
        log_msg("[C-BRIDGE] device not connected");
        return -1;
    }

    log_msg("[C-BRIDGE] starting installation_proxy...");

    installation_proxy_client_t inst = NULL;
    lockdownd_service_descriptor_t service = NULL;

    if (lockdownd_start_service(
            client,
            "com.apple.mobile.installation_proxy",
            &service) != LOCKDOWN_E_SUCCESS) {

        log_msg("[C-BRIDGE] failed to start service");
        return -1;
    }

    if (installation_proxy_client_new(device, service, &inst) != INSTPROXY_E_SUCCESS) {
        log_msg("[C-BRIDGE] installation_proxy init failed");
        return -1;
    }

    log_msg("[C-BRIDGE] sending install request...");

    // ===============================
    // OPTIONS
    // ===============================
    plist_t options = plist_new_dict();
    plist_dict_insert_item(options, "PackageType", plist_new_string("Developer"));

    instproxy_error_t err = installation_proxy_install(
        inst,
        ipa_path,
        options,
        NULL,
        NULL
    );

    if (err != INSTPROXY_E_SUCCESS) {
        log_msg("[C-BRIDGE] install failed");
        plist_free(options);
        installation_proxy_client_free(inst);
        return -1;
    }

    log_msg("[C-BRIDGE] install completed successfully");

    plist_free(options);
    installation_proxy_client_free(inst);

    return 0;
}

// ===============================
// CLEANUP
// ===============================

void signitify_cleanup() {
    if (client) {
        lockdownd_client_free(client);
        client = NULL;
    }

    if (device) {
        idevice_free(device);
        device = NULL;
    }

    log_msg("[C-BRIDGE] cleanup done");
}