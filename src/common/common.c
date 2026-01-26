#include "common.h"

#include <ftw.h>
#include <stdio.h>
#include <stdlib.h>

int handle_traversal(const char *fpath, const struct stat *sb, int tflag, struct FTW *ftwbuf) {
    printf("%s\n", fpath);
    return 0;
}

int main(int argc, char *argv[]) {
    printf("Hello World\n");

    if (nftw("./src", handle_traversal, 20, 0) == -1) {
        perror("nftw threw error");
        exit(EXIT_FAILURE);
    }

    exit(EXIT_SUCCESS);
}

int scanPath(char* path, ScanResults* scan_results) {
    if (nftw(path, handle_traversal, 20, 0) == -1) {
        perror("error while traversing given path");
        return -1;
    }
    return 0;
}
