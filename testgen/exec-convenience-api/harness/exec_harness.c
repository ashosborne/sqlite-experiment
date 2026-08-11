/*
** Test-gen harness — slice exec-convenience-api (run 10).
** Capture driver only (TO_BE_RECORDED); FULL case ids in OBS lines.
** Frees pzErrMsg via sqlite3_free on every path. Public C API only.
*/
#include <stdio.h>
#include <string.h>
#include "sqlite3.h"

typedef struct { int calls; int argc; char argv0[64]; } CbState;

static int cb_record(void *pArg, int argc, char **argv, char **azCol){
  CbState *p = (CbState*)pArg;
  (void)azCol;
  p->calls++;
  p->argc = argc;
  if( argc>0 && argv[0] ){
    snprintf(p->argv0, sizeof(p->argv0), "%s", argv[0]);
  }else{
    snprintf(p->argv0, sizeof(p->argv0), "(null)");
  }
  return 0;
}

static int cb_abort(void *pArg, int argc, char **argv, char **azCol){
  CbState *p = (CbState*)pArg;
  (void)argc; (void)argv; (void)azCol;
  p->calls++;
  return 1; /* abort on first row */
}

int main(void){
  sqlite3 *db = 0;
  char *zErr = 0;
  int rc;

  printf("OBS setup open.rc %d\n", sqlite3_open(":memory:", &db));

  /* exec-convenience-api-001-C001: recording callback returning 0 */
  {
    CbState st; memset(&st, 0, sizeof(st));
    rc = sqlite3_exec(db, "SELECT 1", cb_record, &st, &zErr);
    printf("OBS exec-convenience-api-001-C001 exec.rc %d\n", rc);
    printf("OBS exec-convenience-api-001-C001 cb.calls %d\n", st.calls);
    printf("OBS exec-convenience-api-001-C001 cb.argc %d\n", st.argc);
    printf("OBS exec-convenience-api-001-C001 cb.argv0 %s\n", st.argv0);
    sqlite3_free(zErr); zErr = 0;
  }

  /* exec-convenience-api-001-C002: NULL callback, side effects only */
  rc = sqlite3_exec(db, "SELECT 1", 0, 0, 0);
  printf("OBS exec-convenience-api-001-C002 exec.rc %d\n", rc);

  /* exec-convenience-api-001-C003: callback returns 1 -> abort path */
  {
    CbState st; memset(&st, 0, sizeof(st));
    rc = sqlite3_exec(db, "SELECT 1", cb_abort, &st, &zErr);
    printf("OBS exec-convenience-api-001-C003 exec.rc %d\n", rc);
    printf("OBS exec-convenience-api-001-C003 cb.calls %d\n", st.calls);
    sqlite3_free(zErr); zErr = 0;
  }

  sqlite3_close(db);
  return 0;
}
