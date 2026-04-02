#include "query.h"

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "common.h"

sqlite3 *DB = NULL;
bool     is_db_init = false;

int db_init() {
    sqlite3 *db;
    int      rc = sqlite3_open("sonux.sqlite.db", &db);

    if (rc != SQLITE_OK) {
        fprintf(stderr, "Failed to open DB: %s\n", sqlite3_errmsg(db));
        sqlite3_close(db);
        return 1;
    }

    DB = db;
    is_db_init = true;
    return 0;
}

void db_close() {
    is_db_init = false;
    sqlite3_close(DB);
}

int db_save_files(AudioFile **data_arr, int n) {
    if (!is_db_init) {
        printf("DB is not initialized");
        return 1;
    }

    char *zErrMsg = 0;
    int   rc;

    char *sql_create =
        "CREATE TABLE IF NOT EXISTS audio_files ("
        "path TEXT PRIMARY KEY, "
        "title TEXT, "
        "track_no INTEGER, "
        "disc_no INTEGER, "
        "artist TEXT, "
        "album TEXT, "
        "album_artist TEXT, "
        "release_date TEXT"  // Stored as 'YYYY-MM-DD'
        ");";
    rc = sqlite3_exec(DB, sql_create, NULL, NULL, &zErrMsg);

    if (rc != SQLITE_OK) {
        fprintf(stderr, "SQLITE error: %s\n", zErrMsg);
        sqlite3_free(zErrMsg);
        return 1;
    }

    sqlite3_stmt *stmt;
    char *sql_insert = "INSERT OR REPLACE INTO audio_files VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
    sqlite3_prepare_v2(DB, sql_insert, -1, &stmt, NULL);

    sqlite3_exec(DB, "BEGIN TRANSACTION;", NULL, NULL, NULL);

    for (int i = 0; i < n; i++) {
        AudioFile     file = (*data_arr)[i];
        AudioMetadata *meta = file.audio_metadata;

        sqlite3_bind_text(stmt, 1, file.filepath, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 2, meta->title, -1, SQLITE_STATIC);
        sqlite3_bind_int(stmt, 3, meta->track_no);
        sqlite3_bind_int(stmt, 4, meta->disc_no);
        sqlite3_bind_text(stmt, 5, meta->artist, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 6, meta->album, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 7, meta->album_artist, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 8, meta->release_date, -1, SQLITE_TRANSIENT);

        sqlite3_step(stmt);
        sqlite3_reset(stmt);
    }

    sqlite3_exec(DB, "COMMIT;", NULL, NULL, NULL);
    sqlite3_finalize(stmt);

    return 0;
}
