/*
** Test-gen harness — slice connection-lifecycle-api (run 10).
** Capture driver only (TO_BE_RECORDED): OBS lines carry FULL case ids
** (run-7 collision lesson). Built against the pinned amalgamation
** (overnight/BASELINE.md). Exit code carries no meaning.
*/
#include <stdio.h>
#include "sqlite3.h"

static void obs_int(const char *zCase, const char *zName, int v){
  printf("OBS %s %s %d\n", zCase, zName, v);
}

int main(void){
  sqlite3 *db = 0;
  int rc;

  /* connection-lifecycle-api-001-C001: open ':memory:' */
  rc = sqlite3_open(":memory:", &db);
  obs_int("connection-lifecycle-api-001-C001", "open.rc", rc);
  obs_int("connection-lifecycle-api-001-C001", "db.nonnull", db != 0);

  /* connection-lifecycle-api-001-C002: close that handle (nothing outstanding) */
  rc = sqlite3_close(db);
  obs_int("connection-lifecycle-api-001-C002", "close.rc", rc);

  return 0;
}
