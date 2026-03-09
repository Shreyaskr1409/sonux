#include "tagger.h"

#include <taglib/fileref.h>
#include <taglib/tag.h>
#include <taglib/tpropertymap.h>
#include <taglib/tstringlist.h>

#include <string>

struct Taglib_Handler {
    TagLib::FileRef*     ref;
    TagLib::PropertyMap props;
    std::string          title;
    std::string          album_artist;
};

Taglib_Handler* taglib_open(const char* path) {
    Taglib_Handler* handler = new Taglib_Handler();
    handler->ref = new TagLib::FileRef(path);

    if (handler->ref->isNull()) {
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

const char* taglib_get_album_artist(Taglib_Handler* handler) {
    handler->props = handler->ref->file()->properties();

    TagLib::StringList list = handler->props["ALBUMARTIST"];
    if (list.isEmpty()) list = handler->props["ALBUM ARTIST"];

    handler->album_artist = list.isEmpty() ? "" : list.front().toCString(true);
    return handler->album_artist.c_str();
}
