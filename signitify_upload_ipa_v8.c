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
// CALLBACK STRUCT
// ===============================

typedef void (*log_callback)(const char *msg);
typedef void (*progress_callback)(int percent);

// ===============================
// CONNECT DEVICE
// ===============================

int connect_device(const char *udid) {
    if (idevice_new(&device, udid) != IDEVICE_E_SUCCESS) {
        printf("[V8] device connect failed\n");
        return -1;
    }

    if (lockdownd_client_new_with_handshake(device, &client, "signitify-v8") != LOCKDOWN_E_SUCCESS) {
        printf("[V8] lockdown failed\n");
        return -1;
    }

    printf("[V8] device connected\n");
    return 0;
}

// ===============================
// INTERNAL INSTALL THREAD ARG
// ===============================

typedef struct {
    const char *ipa_path;
    progress_callback progress_cb;
    log_callback log_cb;
} install_args_t;

// ===============================
// SIMULATED PROGRESS POLLER
// (real devices don’t give % directly)
// ===============================

void *progress_simulator(void *arg) {
    install_args_t *args = (install_args_t *)arg;

    for (int i = 0; i <= 100; i += 10) {
        if (args->progress_cb) {
            args->progress_cb(i);
        }

        if (args->log_cb) {
            char msg[128];
            sprintf(msg, "[V8] installing... %d%%", i);
            args->log_cb(msg);
        }

        sleep(1);
    }

    return NULL;
}

// ===============================
// INSTALL CORE
// ===============================

void *install_worker(void *arg) {
    install_args_t *args = (install_args_t *)arg;

    installation_proxy_client_t inst = NULL;
    lockdownd_service_descriptor_t service = NULL;

    args->log_cb("[V8] starting installation_proxy");

    if (lockdownd_start_service(client,
        "com.apple.mobile.installation_proxy",
        &service) != LOCKDOWN_E_SUCCESS) {

        args->log_cb("[V8] failed to start service");
        return NULL;
    }

    if (installation_proxy_client_new(device, service, &inst) != INSTPROXY_E_SUCCESS) {
        args->log_cb("[V8] install proxy init failed");
        return NULL;
    }

    args->log_cb("[V8] sending install request");

    plist_t options = plist_new_dict();
    plist_dict_insert_item(options, "PackageType", plist_new_string("Developer"));

    // start progress thread (fake progress model)
    pthread_t progress_thread;
    pthread_create(&progress_thread, NULL, progress_simulator, args);

    instproxy_error_t res = installation_proxy_install(inst,
        args->ipa_path,
        options,
        NULL,
        NULL
    );

    if (res == INSTPROXY_E_SUCCESS) {
        args->log_cb("[V8] install completed");
        args->progress_cb(100);
    } else {
        args->log_cb("[V8] install failed");
    }

    plist_free(options);
    installation_proxy_client_free(inst);

    return NULL;
}

// ===============================
// PUBLIC API (V8 BRIDGE)
// ===============================

void signitify_install_v8(
    const char *ipa_path,
    progress_callback progress_cb,
    log_callback log_cb
) {
    install_args_t *args = malloc(sizeof(install_args_t));

    args->ipa_path = ipa_path;
    args->progress_cb = progress_cb;
    args->log_cb = log_cb;

    pthread_t thread;
    pthread_create(&thread, NULL, install_worker, args);
    pthread_detach(thread);
}