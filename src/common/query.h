#pragma once

#include <sqlite3.h>
#include <stdbool.h>

#include "common.h"

extern sqlite3 *DB;
extern bool     is_db_init;

int  db_init();
void db_close();
int  db_save_files(AudioFile *data_arr[], int n);
