/*
** Test-gen harness STUB — slice error-status-api (characterization driver).
**
** Stage rules: Test generation writes stubs only. This file has NOT been
** compiled or executed in this run (no library build present). Test execution
** owns building it against the pinned baseline (overnight/BASELINE.md),
** running RECORD, and freezing goldens. Assert mode: TO_BE_RECORDED — this
** driver only CAPTURES observables; it asserts nothing.
**
** Build (Test execution stage):
**   ./configure && make sqlite3.c            # amalgamation of the pinned tree
**   cc -I. error_status_harness.c sqlite3.c -lpthread -ldl -lm -o error_status_harness
**
** Output: one "OBS <case> <name> <value>" line per observable, for scrub+freeze.
*/
#include <stdio.h>
#include "sqlite3.h"

static void obs_int(const char *zCase, const char *zName, int v){
  printf("OBS %s %s %d\n", zCase, zName, v);
}
static void obs_text(const char *zCase, const char *zName, const char *z){
  printf("OBS %s %s %s\n", zCase, zName, z ? z : "(null-pointer)");
}

/* error-status-api-001-C001: errmsg+errcode after failed prepare of invalid SQL */
static int case_C001(void){
  sqlite3 *db = 0;
  sqlite3_stmt *pStmt = 0;
  int rc = sqlite3_open(":memory:", &db);
  obs_int("C001", "open.rc", rc);
  rc = sqlite3_prepare_v2(db, "SELECTT 1", -1, &pStmt, 0);
  /* capture immediately: error state is overwritten by any later call (card edge case) */
  obs_int("C001", "prepare.rc", rc);
  obs_int("C001", "errcode.value", sqlite3_errcode(db));
  obs_int("C001", "extended_errcode.value", sqlite3_extended_errcode(db));
  obs_text("C001", "errmsg.text", sqlite3_errmsg(db));
  sqlite3_finalize(pStmt);   /* pStmt expected NULL on failed prepare; finalize(NULL) is a no-op */
  sqlite3_close(db);
  return 0;
}

/* error-status-api-001-C002: errcode/errmsg on NULL db (guarded static answers) */
static int case_C002(void){
  obs_int("C002", "errcode(NULL).value", sqlite3_errcode(0));
  obs_text("C002", "errmsg(NULL).text", sqlite3_errmsg(0));
  return 0;
}

int main(void){
  case_C001();
  case_C002();
  return 0;  /* capture driver: exit code carries no pass/fail meaning at this stage */
}
