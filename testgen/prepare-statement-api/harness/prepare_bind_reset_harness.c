/*
** Test-gen harness STUB — prepare-statement-api features 001/003/005 (run 6).
**
** Stage rules: Test generation writes stubs; this run performs NO RECORD.
** Compile-checked against the pinned baseline amalgamation (no execution —
** execution/capture is Test execution's job). Assert mode: TO_BE_RECORDED;
** OBS lines are capture output for the future RECORD run; exit code meaningless.
**
** Hard boundary: this driver NEVER touches a statement handle after
** sqlite3_finalize (see prepare-statement-api-002-C003 UAF block).
**
** Build (Test execution stage, per overnight/BASELINE.md):
**   cc -I. prepare_bind_reset_harness.c sqlite3.c -lpthread -ldl -lm -o pbr_harness
*/
#include <stdio.h>
#include <string.h>
#include "sqlite3.h"

static void obs_int(const char *zCase, const char *zName, int v){
  printf("OBS %s %s %d\n", zCase, zName, v);
}
static void obs_text(const char *zCase, const char *zName, const char *z){
  printf("OBS %s %s %s\n", zCase, zName, z ? z : "(null-pointer)");
}

int main(void){
  sqlite3 *db = 0;
  sqlite3_stmt *pStmt = 0;
  const char *zTail = 0;
  int rc;

  obs_int("setup", "open.rc", sqlite3_open(":memory:", &db));

  /* ---- prepare-statement-api-001-C001: prepare valid single statement ---- */
  rc = sqlite3_prepare_v2(db, "SELECT 1", -1, &pStmt, &zTail);
  obs_int("001-C001", "prepare.rc", rc);
  obs_int("001-C001", "stmt.nonnull", pStmt != 0);
  obs_int("001-C001", "pzTail.consumed", zTail != 0 && *zTail == 0);
  sqlite3_finalize(pStmt); pStmt = 0; zTail = 0;

  /* ---- prepare-statement-api-001-C002: whitespace/comment-only SQL ---- */
  rc = sqlite3_prepare_v2(db, "  -- just a comment\n  ", -1, &pStmt, &zTail);
  obs_int("001-C002", "prepare.rc", rc);
  obs_int("001-C002", "stmt.isnull", pStmt == 0);
  obs_text("001-C002", "pzTail.rest", (zTail && *zTail) ? zTail : "(consumed)");
  sqlite3_finalize(pStmt); pStmt = 0; zTail = 0;  /* finalize(NULL) is a no-op */

  /* ---- prepare-statement-api-003-C001: bind_int then ROW ---- */
  rc = sqlite3_prepare_v2(db, "SELECT ?", -1, &pStmt, 0);
  obs_int("003-C001", "prepare.rc", rc);
  obs_int("003-C001", "bind.rc", sqlite3_bind_int(pStmt, 1, 7));
  obs_int("003-C001", "step.rc", sqlite3_step(pStmt));
  obs_int("003-C001", "column_int.value", sqlite3_column_int(pStmt, 0));

  /* ---- prepare-statement-api-003-C002: bind index out of range (same stmt, after reset) ---- */
  sqlite3_reset(pStmt);
  obs_int("003-C002", "bind_oor.rc", sqlite3_bind_int(pStmt, 2, 7));
  sqlite3_finalize(pStmt); pStmt = 0;

  /* ---- prepare-statement-api-005-C001: reset preserves bindings ---- */
  rc = sqlite3_prepare_v2(db, "SELECT ?", -1, &pStmt, 0);
  obs_int("005-C001", "prepare.rc", rc);
  obs_int("005-C001", "bind.rc", sqlite3_bind_int(pStmt, 1, 42));
  obs_int("005-C001", "step_row.rc", sqlite3_step(pStmt));
  obs_int("005-C001", "step_done.rc", sqlite3_step(pStmt));
  obs_int("005-C001", "reset.rc", sqlite3_reset(pStmt));
  obs_int("005-C001", "post_reset_step.rc", sqlite3_step(pStmt));   /* NO re-bind */
  obs_int("005-C001", "column_after_reset.value", sqlite3_column_int(pStmt, 0));
  sqlite3_finalize(pStmt); pStmt = 0;

  /* ---- prepare-statement-api-005-C002: finalize a live statement, rc only ---- */
  rc = sqlite3_prepare_v2(db, "SELECT 1", -1, &pStmt, 0);
  obs_int("005-C002", "prepare.rc", rc);
  obs_int("005-C002", "step_row.rc", sqlite3_step(pStmt));  /* live, mid-row */
  obs_int("005-C002", "finalize.rc", sqlite3_finalize(pStmt));
  pStmt = 0;  /* handle is dead — never touched again (hard boundary) */

  sqlite3_close(db);
  return 0;
}
