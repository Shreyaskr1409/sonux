#include "utils.h"

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <unistd.h>

program_utils *utl_global = NULL;

program_utils *program_utils_create() {
    utl_global = (program_utils *)calloc(1, sizeof(*utl_global));
    if (!utl_global) return NULL;

    utl_global->shutdown_req = 0;
    utl_global->sockfd = -1;
    utl_global->connfd = -1;

    utl_global->sa = (struct sigaction){
        .sa_handler = handle_sig,
        .sa_flags = 0,
    };

    // clears sa_mask from containing garbage value
    sigemptyset(&utl_global->sa.sa_mask);
    // both the interrupts will use same signal_handler since
    // they are registered with same sigaction variable (&utl_global->sa)
    sigaction(SIGINT, &utl_global->sa, NULL);
    sigaction(SIGTERM, &utl_global->sa, NULL);

    return utl_global;
}

void handle_sig(int sig) {
    printf("    => Shutdown signal received.\n");
    utl_global->shutdown_req = 1;

    // close() will not work since it only does the internal cleanup
    // shutdown stops reception and transmission
    if (utl_global->sockfd >= 0) {
        shutdown(utl_global->sockfd, SHUT_RDWR);
        utl_global->sockfd = -1;
    }
}

void program_utils_clear(program_utils *utl) {
    free(utl);
}
