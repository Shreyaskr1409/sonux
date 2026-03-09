#include "common.h"

#include <ftw.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <taglib/tag_c.h>

#include "tag/tagger.h"

static ScanResults *static_scan_results;  // use newScanResults()

static int handle_traversal(const char *fpath, const struct stat *sb, int tflag,
                            struct FTW *ftwbuf) {
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
    printf("\nScan Results:\n");

    ScanResults *scan_results = newScanResults();
    if (argc > 1) {
        scanPath(argv[1], scan_results);

        for (int i = 0; i < scan_results->paths_ptrs_len; i++) {
            if (scan_results->paths[i] == NULL) {  // paths[i] can be NULL
                                                   // this is because paths_ptrs_len
                                                   // is the capacity of the resulting
                                                   // array, not the number of entries
                break;
            }
            printf("%s\n", scan_results->paths[i]);
        }
    }

    AudioFile *data_arr = NULL;
    int n_audio_files = getAudioFiles(scan_results->paths, &data_arr, scan_results->paths_ptrs_len);
    printf("Number of Audio Files: %d\n", n_audio_files);

    AudioMetadata audio_metadata = {0};
    getAudioMetadata(scan_results->paths[1], &audio_metadata);
    printf("\nTitle: %s\nAlbum: %s", audio_metadata.title, audio_metadata.album);
    printf("\nArtist: %s\nAlbum Artist: %s\n", audio_metadata.artist, audio_metadata.album_artist);

    // TODO
    // make separate functions for clenup, it is becoming hard to keep
    // track of all the pointers to be freed
    //
    // also finish up the getAudioFiles and getAudioMetadata functions

    for (int i = 0; i < scan_results->paths_ptrs_len; i++) {
        free(scan_results->paths[i]);
    }
    free(scan_results->paths);
    free(scan_results);

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

int getAudioFiles(char **paths, AudioFile **data_arr, int n) {
    int l = 0;

    printf("\nAudio titles:\n");
    for (int i = 0; i < n; i++) {
        // paths[i] can be NULL
        // this is because paths_ptrs_len is the capacity of the resulting
        // array, not the number of entries.
        if (paths[i] == NULL) {
            break;
        }

        TagLib_File *file = taglib_file_new(paths[i]);
        // in case if path provided is a directory
        if (file == NULL || !taglib_file_is_valid(file)) {
            continue;
        }
        TagLib_Tag *file_tag = taglib_file_tag(file);

        // check if the file is an audio file
        char *audio_title = taglib_tag_title(file_tag);
        if (audio_title == NULL) {
            taglib_file_free(file);
            continue;
        }

        l++;
        printf("%s\n", audio_title);
        // TODO
        // Fill in the AudioFile struct with the paths
        // keep metadata as NULL for now, that will be filled
        // by a separate function call for getAudioMetadata outside
        // the current function

        taglib_tag_free_strings();
        taglib_file_free(file);
    }

    return l;
}

int getAudioMetadata(char *path, AudioMetadata *audio_metadata) {
    // TagLib_File *file = taglib_file_new(path);
    // if (file == NULL || !taglib_file_is_valid(file)) return 1;
    // TagLib_Tag *file_tag = taglib_file_tag(file);
    //
    // memset(audio_metadata, 0, sizeof(*audio_metadata));
    // audio_metadata->title = strdup(taglib_tag_title(file_tag) ?: "");
    // audio_metadata->album = strdup(taglib_tag_album(file_tag) ?: "");
    // audio_metadata->artist = strdup(taglib_tag_artist(file_tag) ?: "");
    // // might need to switch from c to c++ to be able to access other metadata
    // // audio_metadata->album_artist = strdup(taglib_property_get(file, "ALBUMARTIST")[0] ?: "");
    //
    // taglib_tag_free_strings();
    // taglib_file_free(file);
    // return 0;

    Taglib_Handler *handler = taglib_open(path);
    if (!handler) {
        fprintf(stderr, "Failed to open file");
        return 1;
    }

    printf("Title:  %s\n", taglib_get_title(handler));
    audio_metadata->title = strdup(taglib_get_title(handler) ?: "");
    audio_metadata->album_artist = strdup(taglib_get_album_artist(handler) ?: "");
    // audio_metadata->artist = strdup(taglib_tag_artist(file_tag) ?: "");

    taglib_close(handler);
    return 0;
}
