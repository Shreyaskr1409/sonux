#include "common.h"

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
