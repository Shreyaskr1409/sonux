#pragma once

#define _XOPEN_SOURCE 700
#include <ftw.h>

typedef struct __Date {
    int day;
    int month;
    int year;
} Date;

typedef struct __AudioMetadata {
    char* album;
    char* artist;
    char* album_artist;
    int   track_no;
    int   disk_no;
    Date  release_date;
} AudioMetadata;

typedef struct __FileData {
    AudioMetadata* audio_metadata;
    char*          filepath;
} FileData;

typedef struct __ScanResults {
    char** paths;
    long   n;
} ScanResults;

int scanPath(char* path, ScanResults* scan_results);            // exit_status is returned
int getAudioFiles(char* paths[], FileData* data_arr[], int n);  // exit_status is returned
AudioMetadata* getAudioMetadata(char* path);  // could make getVideoMetadata in future
