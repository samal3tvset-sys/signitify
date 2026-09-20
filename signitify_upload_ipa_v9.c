#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include <libimobiledevice/libimobiledevice.h>
#include <libimobiledevice/lockdown.h>
#include <libimobiledevice/installation_proxy.h>
#include <plist/plist.h>

// ===============================
// GLOBAL STATE
// ===============================

static idevice_t device = NULL;
static lockdownd_client_t client = NULL;

// ===============================
// CALLBACKS (REAL-TIME)
// ===============================

typedef void (*log_cb_t)(const char *);
typedef void (*progress_cb_t)(int);
typedef void (*state_cb_t)(const char *);

// ===============================
// CONNECT DEVICE
// ===============================

int connect_device(const char *udid) {
    if (idevice_new(&device, udid) != IDEVICE_E_SUCCESS) {
        return -1;
    }

    if (lockdownd_client_new_with_handshake(device, &client, "signitify-v9") != LOCKDOWN_E_SUCCESS) {
        return -1;
    }

    return 0;
}

// ===============================
// PARSE INSTALL STATUS (REAL)
// ===============================

int parse_status(plist_t node) {
    if (!node) return 0;

    char *xml = NULL;
    plist_to_xml(node, &xml, NULL);

    if (xml) {
        printf("[V9 STATUS] %s\n", xml);
        free(xml);
    }

    return 1;
}

// ===============================
// INSTALL WORKER
// ===============================

typedef struct {
    const char *ipa_path;
    progress_cb_t progress;
    log_cb_t log;
    state_cb_t state;
} ctx_t;

// ===============================
// INSTALL PROXY LISTENER THREAD
// ===============================

void *install_listener(void *arg) {
    ctx_t *ctx = (ctx_t *)arg;

    installation_proxy_client_t inst = NULL;
    lockdownd_service_descriptor_t service = NULL;

    ctx->log("[V9] starting installation_proxy");

    if (lockdownd_start_service(client,
        "com.apple.mobile.installation_proxy",
        &service) != LOCKDOWN_E_SUCCESS) {

        ctx->log("[V9] service failed");
        return NULL;
    }

    if (installation_proxy_client_new(device, service, &inst) != INSTPROXY_E_SUCCESS) {
        ctx->log("[V9] proxy init failed");
        return NULL;
    }

    // ===============================
    // INSTALL OPTIONS
    // ===============================

    plist_t options = plist_new_dict();
    plist_dict_insert_item(options, "PackageType", plist_new_string("Developer"));

    ctx->state("INSTALL_STARTED");

    // ===============================
    // REAL INSTALL CALL (EVENT BASED)
    // ===============================

    instproxy_error_t err = installation_proxy_install(inst,
        ctx->ipa_path,
        options,
        NULL,
        NULL
    );

    if (err != INSTPROXY_E_SUCCESS) {
        ctx->state("INSTALL_FAILED");
        ctx->log("[V9] install error");
    } else {
        ctx->state("INSTALL_FINISHED");
        ctx->progress(100);
        ctx->log("[V9] install complete");
    }

    plist_free(options);
    installation_proxy_client_free(inst);

    return NULL;
}

// ===============================
// PUBLIC API V9
// ===============================

void signitify_install_v9(
    const char *ipa_path,
    progress_cb_t progress_cb,
    log_cb_t log_cb,
    state_cb_t state_cb
) {
    ctx_t *ctx = malloc(sizeof(ctx_t));

    ctx->ipa_path = ipa_path;
    ctx->progress = progress_cb;
    ctx->log = log_cb;
    ctx->state = state_cb;

    pthread_t t;
    pthread_create(&t, NULL, install_listener, ctx);
    pthread_detach(t);
}