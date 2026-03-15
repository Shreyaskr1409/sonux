#pragma once

#define _XOPEN_SOURCE 700
#include <ftw.h>

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
    AudioMetadata* audio_metadata;
    char*          filepath;
} AudioFile;

typedef struct __ScanResults {
    int paths_arr_len;     // capacity of the paths array, not the number of entries
    int    paths_arr_itr;  // contains the highest index at which a path is available
    char** paths;
} ScanResults;

ScanResults* newScanResults();

int scanPath(char* path, ScanResults* scan_results);             // exit_status is returned
int getAudioFiles(char* paths[], AudioFile* data_arr[], int n);  // length of data_arr is returned
int getAudioMetadata(char*          path,
                     AudioMetadata* audio_metadata);  // could make getVideoMetadata in future
