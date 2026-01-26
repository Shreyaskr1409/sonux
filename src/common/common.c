#include "common.h"

#include <ftw.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static ScanResults *static_scan_results;  // use newScanResults()

int handle_traversal(const char *fpath, const struct stat *sb, int tflag, struct FTW *ftwbuf) {
    int *itr = &static_scan_results->paths_ptrs_itr;
    if (static_scan_results->paths_ptrs_len - 1 == *itr) {
        static_scan_results->paths_ptrs_len = static_scan_results->paths_ptrs_len * 2;
        static_scan_results = realloc(static_scan_results, static_scan_results->paths_ptrs_len);
    }

    static_scan_results->paths[*itr] = strdup(fpath);  // fpath is temporary
                                                       // needs to be freed later on
    *itr = *itr + 1;
    return 0;
}

int main(int argc, char *argv[]) {
    printf("Scan Results:\n");

    ScanResults *scan_results = newScanResults();
    scanPath("./src", scan_results);

    for (int i = 0; i < scan_results->paths_ptrs_len; i++) {
        if (scan_results->paths[i] == NULL) {
            break;
        }
        printf("%s\n", scan_results->paths[i]);
    }

    exit(EXIT_SUCCESS);
}

int scanPath(char *path, ScanResults *static_scan_results) {
    if (nftw(path, handle_traversal, 20, 0) == -1) {
        perror("error while traversing given path");
        return -1;
    }
    return 0;
}

ScanResults *newScanResults() {
    static_scan_results = malloc(sizeof(ScanResults));
    memset(static_scan_results, 0, sizeof(ScanResults));

    static_scan_results->paths_ptrs_len = 32;

    char **paths_ptrs = calloc(static_scan_results->paths_ptrs_len, sizeof(char *));
    static_scan_results->paths = paths_ptrs;

    return static_scan_results;
}
