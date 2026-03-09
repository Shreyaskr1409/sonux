#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Taglib_Handler Taglib_Handler;

Taglib_Handler* taglib_open(const char* path);
void            taglib_close(Taglib_Handler* handler);

const char* taglib_get_title(Taglib_Handler* handler);
const char* taglib_get_artist(Taglib_Handler* handler);
const char* taglib_get_album(Taglib_Handler* handler);
const char* taglib_get_album_artist(Taglib_Handler* handler);
int         taglib_get_track_no(Taglib_Handler* handler);
int         taglib_get_disc_no(Taglib_Handler* handler);

#ifdef __cplusplus
}
#endif
