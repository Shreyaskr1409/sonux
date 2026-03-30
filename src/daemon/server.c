#include "server.h"

#include <pthread.h>
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#include "controller.h"
#include "utils.h"

// Represents a TCP connection (server or client).
// Holds socket file descriptor and address info required for accept()/send()/recv().
typedef struct __Connection {
    int                sockfd;
    struct sockaddr_in addr;  // comes from netinet/in.h (requires sys/socket.h)
    socklen_t          addr_len;
} Connection;

// Wrapper struct used to pass multiple arguments to the client handler thread.
// Contains the accepted client connection and the playback controller instance.
typedef struct __HandleClientArgs {
    Connection         *conn;
    PlaybackController *ctl;
} HandleClientArgs;

// Static variables or forward declarations to this file to be declared below:
static void handle_conn(PlaybackController *ctl, Connection *server_conn);

/*
Initializes and runs the TCP server.
- Creates socket, sets reuse options, binds to PORT
- Stores socket globally for shutdown handling
- Delegates connection handling to handle_conn()
- Cleans up socket on shutdown
*/
void run_server(PlaybackController *ctl) {
    Connection server_conn;

    server_conn.sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (server_conn.sockfd < 0) {
        perror("Error while creating the socket");
        return;
    }

    // in case program is restarted after a crash
    // bypasses the waiting time for the socket to become reusable
    int opt = 1;
    setsockopt(server_conn.sockfd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    server_conn.addr.sin_family = AF_INET;
    server_conn.addr.sin_addr.s_addr = INADDR_ANY;
    server_conn.addr.sin_port = htons(PORT);

    server_conn.addr_len = sizeof(server_conn.addr);

    if (bind(server_conn.sockfd, (struct sockaddr *)&server_conn.addr, server_conn.addr_len) != 0) {
        perror("error while binding to the socket");
        close(server_conn.sockfd);  // comes from unistd.h
        return;
    }

    // keep in mind
    utl_global->sockfd = server_conn.sockfd;

    handle_conn(ctl, &server_conn);

    // in case there was an interrupt signal, sockfd would be assigned as 0
    // else it would be open as sockfd > 0
    if (utl_global->sockfd > 0) {
        close(server_conn.sockfd);
    }
    return;
}

/*
Main server loop that listens for and accepts incoming client connections.
- Calls listen() and blocks on accept()
- For each client, allocates connection struct
- Spawns a detached worker thread (handle_client) per connection
- Continues until shutdown flag is set
*/
static void handle_conn(PlaybackController *ctl, Connection *server_conn) {
    // Should first prepare the gst pipeline, put it in the PAUSED state
    // and then listen to connections

    if (listen(server_conn->sockfd, 4)) {
        perror("error while listening to the socket");
        return;
    }

    printf("Server listening on port %d...\n", PORT);
    printf("------------\n");

    while (utl_global->shutdown_req == 0) {
        Connection *client_conn = malloc(sizeof(*client_conn));
        client_conn->addr_len = sizeof(client_conn->addr);

        // accept is going to block till a connection is made
        // OR
        // till the socket is closed
        // (thus in case of shutdown, the server socket needs to be closed)
        client_conn->sockfd = accept(server_conn->sockfd, (struct sockaddr *)&client_conn->addr,
                                     &client_conn->addr_len);

        if (client_conn->sockfd < 0) {
            if (utl_global->shutdown_req == 0) perror("Accept failed");
            close(client_conn->sockfd);
            free(client_conn);
            continue;
        }

        printf("INFO: Request recieved.\n");

        HandleClientArgs *args = malloc(sizeof(HandleClientArgs));
        *args = (HandleClientArgs){client_conn, ctl};

        pthread_t worker_thread;
        pthread_create(&worker_thread, NULL, handle_client, (void *)args);
        // remember to make the handle_client have a timeout later
        pthread_detach(worker_thread);
    }
}

/*
Worker thread function to handle a single client request.
- Receives command from client (PLAY, PAUSE, RESUME)
- Parses input using strtok_r
- Invokes appropriate controller action
- Sends response back to client
- Cleans up connection and allocated resources
*/
void *handle_client(void *arg) {
    HandleClientArgs *args = (HandleClientArgs *)arg;

    char buffer[BUFFER_SIZE + 1];
    // memset was needed because when i was printing buffer, i was
    // getting a lot of garbage value printed after the first \n
    // like if buffer was RESUME/n once, the next time it printed PLAY/nE/n
    memset(buffer, 0, sizeof(buffer));

    ssize_t bytes_received = recv(args->conn->sockfd, buffer, BUFFER_SIZE, 0);
    if (bytes_received == 0) {
        const char *resp;
        resp = "NO MESSAGE";
        send(args->conn->sockfd, resp, strlen(resp), 0);
        goto cleanup;
    }
    printf("%s", buffer);

    char *saveptr = NULL;
    char *token = strtok_r(buffer, " \n\r", &saveptr);

    printf("TOKEN: %s\n", token);

    if (strcmp(token, "PLAY") == 0) {
        char *uri = strtok_r(NULL, "\n\r", &saveptr);
        printf("PLAYING\n");
        controller_play_track(args->ctl, uri);
    } else if (strcmp(token, "PAUSE") == 0) {
        printf("PAUSING\n");
        controller_pause_track(args->ctl);
    } else if (strcmp(token, "RESUME") == 0) {
        printf("RESUMING\n");
        controller_resume_track(args->ctl);
    } else {
        printf("INVALID REQUEST %s\n", token);
    }

    const char *resp;
    resp = "SUCCESS\n";
    send(args->conn->sockfd, resp, strlen(resp), 0);

cleanup:
    printf("client_sock closed (0 or -1): %d\n", close(args->conn->sockfd));
    free(args->conn);
    free(args);
    printf("------------\n");

    return NULL;
}
