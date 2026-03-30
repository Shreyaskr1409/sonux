#include "tagger.h"

#include <taglib/fileref.h>
#include <taglib/tag.h>
#include <taglib/tpropertymap.h>
#include <taglib/tstringlist.h>

#include <cstdio>
#include <cstdlib>
#include <string>

struct Taglib_Handler {
    TagLib::FileRef*    ref;
    TagLib::PropertyMap props;
    std::string         title;
    std::string         album;
    std::string         artist;
    std::string         album_artist;
    int                 track_no;
    int                 disc_no;
};

Taglib_Handler* taglib_open(const char* path) {
    Taglib_Handler* handler = new Taglib_Handler();
    handler->ref = new TagLib::FileRef(path);

    if (handler->ref->isNull()) {
        printf("File can't be opened\n");
        delete handler->ref;
        delete handler;
        return nullptr;
    }

    return handler;
}

void taglib_close(Taglib_Handler* handler) {
    if (!handler) return;
    delete handler->ref;
    delete handler;
}

const char* taglib_get_title(Taglib_Handler* handler) {
    handler->title = handler->ref->tag()->title().toCString(true);
    return handler->title.c_str();
}

const char* taglib_get_artist(Taglib_Handler* handler) {
    handler->artist = handler->ref->tag()->artist().toCString(true);
    return handler->artist.c_str();
}

const char* taglib_get_album(Taglib_Handler* handler) {
    handler->album = handler->ref->tag()->album().toCString(true);
    return handler->album.c_str();
}

const char* taglib_get_album_artist(Taglib_Handler* handler) {
    handler->props = handler->ref->file()->properties();

    TagLib::StringList list = handler->props["ALBUMARTIST"];
    if (list.isEmpty()) list = handler->props["ALBUM ARTIST"];

    handler->album_artist = list.isEmpty() ? "" : list.front().toCString(true);
    return handler->album_artist.c_str();
}

int taglib_get_track_no(Taglib_Handler* handler) {
    handler->track_no = handler->ref->tag()->track();
    return handler->track_no;
}

int taglib_get_disc_no(Taglib_Handler* handler) {
    handler->props = handler->ref->file()->properties();

    TagLib::StringList list = handler->props["DISCNUMBER"];
    if (list.isEmpty()) return 0;
    return atoi(list.front().toCString(false));
}
