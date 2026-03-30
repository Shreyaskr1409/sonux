#include <gstreamer-1.0/gst/gst.h>
#include <pthread.h>
#include <stdio.h>

#include "controller.h"
#include "server.h"
#include "utils.h"

#ifdef __APPLE__
#include <TargetConditionals.h>
#endif

// server_routine launches server into a separate thread. This thread closes at
// a Fatal signal and closes GRACEFULLY when a graceful shutdown is asked for
// through Interrupts.
void *server_routine(void *arg) {
    printf("Server routine executing in a separate thread...\n");

    PlaybackController *ctl = arg;
    run_server(ctl);

    return NULL;
}

// Main entry point function. The arguments passed to the binary will be passed
// down to the gstreamer initializer function. Gstreamer flags can be directly
// passed through the binary.
int main(int argc, char *argv[]) {
    printf("Hello World\n");

    // here *utl = *utl_global
    program_utils *utl = program_utils_create();

    gst_init(&argc, &argv);
    printf("Using %s as player.\n", gst_version_string());

    PlaybackController ctl;
    playback_controller_init(&ctl);

    if (pthread_create(&utl->server_thread, NULL, server_routine, &ctl) != 0) {
        perror("Failed to create a thread");
    }

    if (pthread_join(utl->server_thread, NULL) != 0) {
        perror("Failed to join thread");
    }
    printf("Thread joined. Clearing all the used data.\n");

    playback_controller_clear(&ctl);
    program_utils_clear(utl);
    return 0;
}
