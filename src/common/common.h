#pragma once

#define _XOPEN_SOURCE 700
#include <ftw.h>
#include <stdbool.h>

typedef struct __Date {
    int day;
    int month;
    int year;
} Date;

typedef struct __AudioMetadata {
    char* title;
    char* album;
    char* artist;
    char* album_artist;
    int   track_no;
    int   disc_no;
    Date  release_date;
} AudioMetadata;

typedef struct __AudioFile {
    AudioMetadata* audio_metadata;  // anything like this would be a pointer because
                                    // i want to be able to change these things during runtime
    char* filepath;
} AudioFile;

typedef struct __PathScanResults {
    int    paths_arr_len;  // capacity of the paths array, not the number of entries
    int    paths_arr_itr;  // contains the highest index at which a path is available
    char** paths;
    bool   isAudioFile;
} PathScanResults;

PathScanResults* newPathScanResults();

int scanPath(char* dir_path, PathScanResults* path_scan_results);  // exit_status is returned
int getAudioFiles(char* paths[], AudioFile* data_arr[], int n);    // length of data_arr is returned
int getAudioMetadata(char*          path,
                     AudioMetadata* audio_metadata);  // could make getVideoMetadata in future
