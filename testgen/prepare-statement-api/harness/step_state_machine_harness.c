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

  /* C003 REMOVED (Test execution run 5, operator charter): stepping a finalized
  ** statement is use-after-free even under SQLITE_ENABLE_API_ARMOR (armor guards
  ** NULL pointers, not freed handles). Case BLOCKED in TRACEABILITY — undefined
  ** behaviour is never captured or frozen. Possible future replacement (NOT this
  ** run): sqlite3_step(NULL) as a defined guarded-misuse probe. */
  sqlite3_finalize(pStmt);

  sqlite3_close(db);
  return 0;
}
