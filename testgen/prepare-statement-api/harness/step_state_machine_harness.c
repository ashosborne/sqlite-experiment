/*
** Test-gen harness STUB — slice prepare-statement-api (step state machine).
**
** NOT compiled/executed this run (no library build present). Test execution
** builds against the pinned baseline (overnight/BASELINE.md). Assert mode:
** TO_BE_RECORDED — capture only, no asserts, exit code meaningless.
**
** Build (Test execution stage):
**   ./configure && make sqlite3.c
**   cc -I. step_state_machine_harness.c sqlite3.c -lpthread -ldl -lm -o step_harness
*/
#include <stdio.h>
#include "sqlite3.h"

static void obs_int(const char *zCase, const char *zName, int v){
  printf("OBS %s %s %d\n", zCase, zName, v);
}

int main(void){
  sqlite3 *db = 0;
  sqlite3_stmt *pStmt = 0;
  int rc;

  obs_int("C001", "open.rc", sqlite3_open(":memory:", &db));

  /* C001: prepare + ROW + column + DONE */
  rc = sqlite3_prepare_v2(db, "SELECT 1", -1, &pStmt, 0);
  obs_int("C001", "prepare.rc", rc);
  obs_int("C001", "step1.rc", sqlite3_step(pStmt));
  obs_int("C001", "column_int.value", sqlite3_column_int(pStmt, 0));
  obs_int("C001", "step2.rc", sqlite3_step(pStmt));

  /* C002: third step after DONE, no reset (same statement, same state) */
  obs_int("C002", "step3_after_done.rc", sqlite3_step(pStmt));

  /* C003: step after finalize.
  ** CAVEAT (see CASE-003.md): guarded MISUSE only with SQLITE_ENABLE_API_ARMOR;
  ** on an unarmored build this is use-after-free — Test execution must check the
  ** compileoption fingerprint and mark the case BLOCKED instead of capturing UB. */
  sqlite3_finalize(pStmt);
  obs_int("C003", "step_after_finalize.rc", sqlite3_step(pStmt));

  sqlite3_close(db);
  return 0;
}
