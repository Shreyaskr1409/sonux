#include "common.h"

#include <ftw.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <taglib/tag_c.h>
#include <time.h>

#include "query.h"
#include "tag/tagger.h"

// Static variables to this file to be declared below:
static PathScanResults *static_scan_results;  // use newScanResults()
static FILE            *logfile;

/*
This method handles directory traversal as per the template which <ftw.h> requires.
While stat* sb is not being used currently, it is an important asset for determining
the stats such as file_size of the file, as well as date of addition to the library
or database (which I will be using later on as the project progresses).
*/
static int handle_traversal(const char *fpath, const struct stat *sb, int tflag,
                            struct FTW *ftwbuf) {
    int *itr = &static_scan_results->paths_arr_itr;

    if (static_scan_results->paths_arr_len - 1 == *itr) {
        static_scan_results->paths_arr_len = static_scan_results->paths_arr_len * 2;

        char **new_paths = realloc(static_scan_results->paths,
                                   static_scan_results->paths_arr_len * sizeof(char *));

        // set allocated values to 0 (new paths pointers become NULL pointers)
        memset(new_paths + *itr, 0, (static_scan_results->paths_arr_len / 2) * sizeof(char *));

        static_scan_results->paths = new_paths;
    }

    fprintf(logfile, "%d - %s\n", *itr, fpath);
    fflush(logfile);
    static_scan_results->paths[*itr] = strdup(fpath);  // fpath is temporary
                                                       // needs to be freed later on
    *itr = *itr + 1;
    return 0;
}

/*
Main function for testing out common utilities.

This will be removed later on when this module will be complete. For now what this module does is:
- Scans for all possible paths inside a directory provided in args
- Logs out all traversed paths in a scan.log file
- From the array of traversed path, AudioFile(s) are extracted
- AudioFile will contain the path as well as the metadata
*/
int main(int argc, char *argv[]) {
    logfile = fopen("scan.log", "a");
    if (!logfile) exit(EXIT_FAILURE);
    time_t now = time(NULL);
    fprintf(logfile, "\n%s%s\n", ctime(&now), "Logging starts");

    printf("\nScan Results:\n");

    PathScanResults *scan_results = newPathScanResults();
    if (argc > 1) {
        scanPath(argv[1], scan_results);
    }

    AudioFile *data_arr = NULL;
    int n_audio_files = getAudioFiles(scan_results->paths, &data_arr, scan_results->paths_arr_itr);

    printf("\nScan results:\n");
    printf("Number of Paths scanned: %d\n", scan_results->paths_arr_itr);
    printf("Number of Audio Files: %d\n", n_audio_files);

    // // Scan output logging:
    // for (int i = 0; i < n_audio_files; i++) {
    //     AudioFile *f = &data_arr[i];
    //
    //     printf("\n%d.", i + 1);
    //     printf("\nPath: %s\n", f->filepath);
    //     printf("\nDisc No: %d\nTrack No: %d", f->audio_metadata->disc_no,
    //            f->audio_metadata->track_no);
    //     printf("\nTitle: %s\nAlbum: %s", f->audio_metadata->title, f->audio_metadata->album);
    //     printf("\nArtist: %s\nAlbum Artist: %s\n", f->audio_metadata->artist,
    //            f->audio_metadata->album_artist);
    // }

    db_init();
    db_save_files(&data_arr, n_audio_files);
    db_close();

    for (int i = 0; i < scan_results->paths_arr_len; i++) {
        free(scan_results->paths[i]);
    }
    free(scan_results->paths);
    free(scan_results);
    fclose(logfile);

    exit(EXIT_SUCCESS);
}

/*
scanPath() uses nftw() to traverse through the provided directory according to the
handle_traversal() function.
More information on handle_traversal() is provided before code for the function.
*/
int scanPath(char *dir_path, PathScanResults *path_scan_results) {
    if (nftw(dir_path, handle_traversal, 20, 0) == -1) {
        perror("error while traversing given path");
        return -1;
    }
    return 0;
}

/*
newPathScanResults is a simple allocator for initializing a PathScanResults struct.
This became as a requirement because I left too many pointers inside the struct which
needed initialization before they come into use.
*/
PathScanResults *newPathScanResults() {
    static_scan_results = malloc(sizeof(PathScanResults));
    memset(static_scan_results, 0, sizeof(PathScanResults));

    static_scan_results->paths_arr_len = 32;

    char **paths_ptrs = calloc(static_scan_results->paths_arr_len, sizeof(char *));
    static_scan_results->paths = paths_ptrs;

    return static_scan_results;
}

/*
getAudioFiles() filters out actual AudioFiles on basis of (whether they have a title or not).
While this is not an ideal filteration criteria, it works for me since I want to handle well
tagged music library. **data_arr (pointer to array) needs to be passed as a NULL pointer since
the actual address will be dynamically allocated inside this struct.

The resizing of data_arr happens dynamically by initial capacity of 32 and multiplication factor
of 2.
*/
int getAudioFiles(char **paths, AudioFile **data_arr, int n) {
    int count = 0;

    int capacity = 32;
    *data_arr = malloc(capacity * sizeof(AudioFile));

    printf("\nAudio titles:\n");
    for (int i = 0; i < n; i++) {
        TagLib_File *file = taglib_file_new(paths[i]);
        // in case if path provided is a directory
        if (file == NULL || !taglib_file_is_valid(file)) {
            continue;
        }
        TagLib_Tag *file_tag = taglib_file_tag(file);

        // check if the file is an audio file
        // for now, just verification for whether the file
        // has audio_title field or not, will work.
        char *audio_title = taglib_tag_title(file_tag);
        if (audio_title == NULL) {
            taglib_file_free(file);
            continue;
        }

        if (count >= capacity) {
            capacity *= 2;
            *data_arr = realloc(*data_arr, capacity * sizeof(AudioFile));
        }

        (*data_arr)[count].filepath = strdup(paths[i]);
        (*data_arr)[count].audio_metadata = malloc(sizeof(AudioMetadata));

        getAudioMetadata(paths[i], (*data_arr)[count].audio_metadata);

        count++;
        printf("%s\n", audio_title);

        taglib_tag_free_strings();
        taglib_file_free(file);
    }

    return count;
}

/*
getAudioMetadata() fills up audio_metadata struct (assuming it is already allocated) with
audio metadata from TagLib C wrapper over the C++ API. This was done because TagLib's official
C wrapper, atleast the version I have installed, does not provide support for accessing property
maps. C++ on the other hand, has the support. The wrappers can be found in the "./tag/" subdirectory
relative to the current file directory.

If any property is not found, then it will be either filled as a blank string or 0;
*/
int getAudioMetadata(char *path, AudioMetadata *audio_metadata) {
    Taglib_Handler *handler = taglib_open(path);
    if (!handler) {
        fprintf(stderr, "Failed to open file");
        return 1;
    }
    memset(audio_metadata, 0, sizeof(*audio_metadata));

    audio_metadata->title = strdup(taglib_get_title(handler) ?: "");
    audio_metadata->album_artist = strdup(taglib_get_album_artist(handler) ?: "");
    audio_metadata->artist = strdup(taglib_get_artist(handler) ?: "");
    audio_metadata->album = strdup(taglib_get_album(handler) ?: "");
    audio_metadata->track_no = taglib_get_track_no(handler);
    audio_metadata->disc_no = taglib_get_disc_no(handler);
    audio_metadata->release_date = strdup("");

    taglib_close(handler);
    return 0;
}
