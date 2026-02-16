#pragma once

#include <pthread.h>
#include <signal.h>
#include <stdlib.h>

typedef struct __program_utils {
    // shutdown req used to gracefully stop all running
    // processes in a thread
    volatile sig_atomic_t shutdown_req;
    struct sigaction      sa;

    int sockfd;
    int connfd;

    // all running threads need to be mentioned here in this struct
    pthread_t server_thread;
} program_utils;

// this is used as an extern so that sockfd and connfd
// can be assigned directly, but I might remove this and
// add a function instead
extern program_utils *utl_global;

void           handle_sig(int sig);
program_utils *program_utils_create();
void           program_utils_clear(program_utils *utl);
