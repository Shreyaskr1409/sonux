#include "controller.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#include "glib-object.h"
#include "glib.h"
#include "gst/gstelement.h"
#include "gst/gstelementfactory.h"
#include "gst/gstobject.h"
#include "gst/gsturi.h"

/*
Creates and returns a new GStreamer "playbin" element.
This is the main high-level playback element used for audio playback.
The element is initialized but not yet configured with any URI or state.
*/
GstElement *create_idle_playbin() {
    GstElement *playbin;  // made a local variable because it will dynamically allocated

    playbin = gst_element_factory_make("playbin", "main-player");
    if (!playbin) {
        g_error("Failed to create playbin.");
        return NULL;
    }

    return playbin;
}

/*
Initializes the PlaybackController structure.
- Resets all fields to zero
- Creates a playbin instance for playback
- Initializes synchronization primitives (mutex and condition variable)
- Retrieves the GStreamer bus for message handling
- Sets default values for URIs and callbacks
*/
void playback_controller_init(PlaybackController *ctl) {
    memset(ctl, 0, sizeof(*ctl));

    ctl->playbin = create_idle_playbin();
    if (!ctl->playbin) {
        g_error("Failed to initialize PlaybackController: playbin is NULL");
        return;
    }

    ctl->current_uri = NULL;
    ctl->upcoming_uri = NULL;

    g_mutex_init(&ctl->lock);
    g_cond_init(&ctl->cond);

    ctl->bus = gst_element_get_bus(ctl->playbin);
    ctl->bus_thread = NULL;
    ctl->track_finished_cb = NULL;
    ctl->error_cb = NULL;
}

/*
Cleans up resources associated with the PlaybackController.
- Unrefs GStreamer objects (playbin and bus)
- Clears mutex and condition variable
Should be called before destroying the controller to avoid memory/resource leaks.
*/
void playback_controller_clear(PlaybackController *ctl) {
    if (!ctl) return;

    if (ctl->bus) gst_object_unref(ctl->bus);

    if (ctl->playbin) gst_object_unref(ctl->playbin);

    g_mutex_clear(&ctl->lock);
    g_cond_clear(&ctl->cond);
}

/*
Starts playback of a given track.
- Accepts either a file path or "DEFAULT" keyword
- Validates file existence using stat()
- Converts file path to a GStreamer-compatible URI
- Resets playbin to NULL state before setting new URI
- Transitions playbin to PLAYING state to begin playback
*/
void controller_play_track(PlaybackController *ctl, char *arg) {
    struct stat st;
    char       *uri = NULL;
    if (strcmp(arg, "DEFAULT") == 0) {
        char *alt_uri = "file:///home/shrey/Music/Albums/3D Country/02 3D Country.mp3";
        uri = alt_uri;
    } else {
        if (stat(arg, &st) != 0) {
            printf("Invalid URI %s\n", uri);
            return;
        }
        // Remember to replaye null with a GError instance later
        uri = gst_filename_to_uri(arg, NULL);
        if (!uri) {
            fprintf(stderr, "Failed to convert path to URI.\n");
            return;
        }
    }

    printf("URI: %s\n", uri);

    gst_element_set_state(ctl->playbin, GST_STATE_NULL);
    g_object_set(G_OBJECT(ctl->playbin), "uri", uri, NULL);
    gst_element_set_state(ctl->playbin, GST_STATE_PLAYING);
}

// Pauses the currently playing track by setting playbin to PAUSED state.
void controller_pause_track(PlaybackController *ctl) {
    gst_element_set_state(ctl->playbin, GST_STATE_PAUSED);
}

// Resumes playback of the current track by setting playbin back to PLAYING state.
void controller_resume_track(PlaybackController *ctl) {
    gst_element_set_state(ctl->playbin, GST_STATE_PLAYING);
}
