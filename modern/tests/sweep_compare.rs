//! GENERATED (pack v10): completion-sweep executor replays — no script_table.
mod util;
use util::compare_script;

#[test]
fn engine_datetime_001_C001() {
    compare_script("engine-datetime", "engine-datetime-001", "C001", "SELECT time('2026-08-11 04:05:06'), julianday('2000-01-01 12:00:00');");
}

#[test]
fn engine_datetime_001_C002() {
    compare_script("engine-datetime", "engine-datetime-001", "C002", "SELECT date('2026-08-11','start of month'), date('2026-08-11','start of year'), datetime('2026-08-11 04:05:06','start of day');");
}

#[test]
fn engine_datetime_001_C003() {
    compare_script("engine-datetime", "engine-datetime-001", "C003", "SELECT date('2026-08-11','-40 days'), datetime('2026-08-11 04:05:06','+2 hours','+30 minutes','+15 seconds');");
}

#[test]
fn engine_datetime_001_C004() {
    compare_script("engine-datetime", "engine-datetime-001", "C004", "SELECT strftime('%H:%M:%S','2026-08-11 04:05:06'), strftime('%s','2001-01-01'), strftime('%w','2026-08-11');");
}

#[test]
fn engine_datetime_001_C005() {
    compare_script("engine-datetime", "engine-datetime-001", "C005", "SELECT datetime(978307200,'unixepoch'), unixepoch('2026-08-11 00:00:00');");
}

#[test]
fn engine_datetime_001_C006() {
    compare_script("engine-datetime", "engine-datetime-001", "C006", "SELECT date('2026-08-16','weekday 0'), date('2026-08-11','weekday 2');");
}

#[test]
fn engine_datetime_001_C007() {
    compare_script("engine-datetime", "engine-datetime-001", "C007", "SELECT datetime('2024-02-29','+1 year'), date('2026-03-31','-1 month');");
}

#[test]
fn engine_datetime_001_C008() {
    compare_script("engine-datetime", "engine-datetime-001", "C008", "SELECT timediff('2026-08-11 10:30:00','2026-08-11 09:15:30'), timediff('2025-08-10','2026-08-11');");
}

#[test]
fn engine_setops_001_C001() {
    compare_script("engine-setops", "engine-setops-001", "C001", "CREATE TABLE s1(a); INSERT INTO s1 VALUES(1),(2),(3); CREATE TABLE s2(a); INSERT INTO s2 VALUES(2),(3),(4); SELECT a FROM s1 INTERSECT SELECT a FROM s2 ORDER BY a;");
}

#[test]
fn engine_setops_001_C002() {
    compare_script("engine-setops", "engine-setops-001", "C002", "CREATE TABLE s1(a); INSERT INTO s1 VALUES(1),(2),(3); CREATE TABLE s2(a); INSERT INTO s2 VALUES(2),(3),(4); SELECT a FROM s1 EXCEPT SELECT a FROM s2 ORDER BY a;");
}

#[test]
fn engine_setops_001_C003() {
    compare_script("engine-setops", "engine-setops-001", "C003", "SELECT x FROM (SELECT 1 AS x UNION ALL SELECT 1 UNION ALL SELECT 2) INTERSECT SELECT 1;");
}

#[test]
fn engine_setops_001_C004() {
    compare_script("engine-setops", "engine-setops-001", "C004", "CREATE TABLE s3(a); INSERT INTO s3 VALUES(5); SELECT a FROM s3 EXCEPT SELECT 5;");
}

#[test]
fn engine_constraints_001_C001() {
    compare_script("engine-constraints", "engine-constraints-001", "C001", "CREATE TABLE c(a CHECK(a>0)); INSERT INTO c VALUES(-1);");
}

#[test]
fn engine_constraints_001_C002() {
    compare_script("engine-constraints", "engine-constraints-001", "C002", "CREATE TABLE c(a CHECK(a>0)); INSERT INTO c VALUES(5); INSERT OR IGNORE INTO c VALUES(-1); SELECT count(*) FROM c;");
}

#[test]
fn engine_constraints_001_C003() {
    compare_script("engine-constraints", "engine-constraints-001", "C003", "CREATE TABLE nn(a NOT NULL); INSERT INTO nn VALUES(NULL);");
}

#[test]
fn engine_constraints_001_C004() {
    compare_script("engine-constraints", "engine-constraints-001", "C004", "CREATE TABLE cf(a CHECK(a<10)); INSERT INTO cf VALUES(1); INSERT OR FAIL INTO cf VALUES(2),(20),(3); SELECT count(*) FROM cf;");
}

#[test]
fn engine_constraints_001_C005() {
    compare_script("engine-constraints", "engine-constraints-001", "C005", "CREATE TABLE n1(a); CREATE TABLE n2(a); SELECT a FROM n1, n2;");
}

#[test]
fn engine_constraints_001_C006() {
    compare_script("engine-constraints", "engine-constraints-001", "C006", "CREATE TABLE t1(a); SELECT nosuch FROM t1;");
}

#[test]
fn engine_constraints_001_C007() {
    compare_script("engine-constraints", "engine-constraints-001", "C007", "PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE ch(pid REFERENCES p(id) ON DELETE SET NULL); INSERT INTO p VALUES(1); INSERT INTO ch VALUES(1); DELETE FROM p WHERE id=1; SELECT count(*), count(pid) FROM ch;");
}

#[test]
fn engine_constraints_001_C008() {
    compare_script("engine-constraints", "engine-constraints-001", "C008", "PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE RESTRICT); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2 WHERE id=1;");
}

#[test]
fn engine_views_001_C001() {
    compare_script("engine-views", "engine-views-001", "C001", "CREATE TABLE vt(a INTEGER, b TEXT); INSERT INTO vt VALUES(1,'x'),(2,'y'); CREATE VIEW v AS SELECT a, b FROM vt WHERE a > 1; SELECT a, b FROM v;");
}

#[test]
fn engine_views_001_C002() {
    compare_script("engine-views", "engine-views-001", "C002", "CREATE TABLE vt(a INTEGER); INSERT INTO vt VALUES(3); CREATE VIEW v AS SELECT a*2 AS d FROM vt; SELECT d FROM v; SELECT count(*) FROM sqlite_master WHERE type='view'; DROP VIEW v; SELECT count(*) FROM sqlite_master WHERE type='view';");
}

#[test]
fn engine_views_001_C003() {
    compare_script("engine-views", "engine-views-001", "C003", "CREATE TABLE vt(a INTEGER); CREATE VIEW v AS SELECT a FROM vt; INSERT INTO v VALUES(1);");
}

#[test]
fn engine_triggers_001_C001() {
    compare_script("engine-triggers", "engine-triggers-001", "C001", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER td AFTER DELETE ON t BEGIN INSERT INTO lg VALUES(old.a); END; INSERT INTO t VALUES(7),(8); DELETE FROM t WHERE a=7; SELECT v FROM lg;");
}

#[test]
fn engine_triggers_001_C002() {
    compare_script("engine-triggers", "engine-triggers-001", "C002", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(o INTEGER, n INTEGER); CREATE TRIGGER tu AFTER UPDATE ON t BEGIN INSERT INTO lg VALUES(old.a, new.a); END; INSERT INTO t VALUES(5); UPDATE t SET a=9 WHERE a=5; SELECT o, n FROM lg;");
}

#[test]
fn engine_triggers_001_C003() {
    compare_script("engine-triggers", "engine-triggers-001", "C003", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tw AFTER INSERT ON t WHEN new.a > 10 BEGIN INSERT INTO lg VALUES(new.a); END; INSERT INTO t VALUES(5),(15); SELECT count(*), max(v) FROM lg;");
}

#[test]
fn engine_triggers_001_C004() {
    compare_script("engine-triggers", "engine-triggers-001", "C004", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v TEXT); CREATE TRIGGER tb BEFORE INSERT ON t BEGIN INSERT INTO lg VALUES('B'); END; CREATE TRIGGER ta AFTER INSERT ON t BEGIN INSERT INTO lg VALUES('A'); END; INSERT INTO t VALUES(1); SELECT v FROM lg;");
}

#[test]
fn engine_funcs_001_C001() {
    compare_script("engine-funcs", "engine-funcs-001", "C001", "SELECT printf('%5d|%-4d|%05.1f|%x|%X', 42, 7, 3.14159, 255, 255), printf('%.3s|%c|%o', 'hello', 65, 8);");
}

#[test]
fn engine_funcs_001_C002() {
    compare_script("engine-funcs", "engine-funcs-001", "C002", "SELECT printf('%e|%g|%+d', 12345.678, 0.0001, 5), format('%08.3f', 2.5);");
}

#[test]
fn engine_funcs_001_C003() {
    compare_script("engine-funcs", "engine-funcs-001", "C003", "SELECT decimal_sub('5.25','1.1'), decimal_mul('1.25','4'), decimal_add('0.1','0.2');");
}

#[test]
fn engine_funcs_001_C004() {
    compare_script("engine-funcs", "engine-funcs-001", "C004", "SELECT 'Uryyb' = 'Hello' COLLATE rot13, 'abc' < 'abd' COLLATE rot13, rot13('Gung');");
}

#[test]
fn engine_funcs_001_C005() {
    compare_script("engine-funcs", "engine-funcs-001", "C005", "SELECT round(2.5), round(2.567,2), trim('  x  '), replace('aXbXc','X','-'), instr('hello','ll');");
}

#[test]
fn engine_funcs_001_C006() {
    compare_script("engine-funcs", "engine-funcs-001", "C006", "SELECT min(3,1,2), max(3,1,2), sign(-5), sign(0), char(65,66,67), unhex('4142') = X'4142';");
}

#[test]
fn engine_funcs_001_C007() {
    compare_script("engine-funcs", "engine-funcs-001", "C007", "SELECT concat('a','b','c'), concat_ws('-','x','y'), octet_length('abc'), unicode('A'), ltrim('xxay','x'), rtrim('yaxx','x');");
}

#[test]
fn engine_pragma_001_C001() {
    compare_script("engine-pragma", "engine-pragma-001", "C001", "PRAGMA foreign_keys; PRAGMA foreign_keys=ON; PRAGMA foreign_keys; PRAGMA busy_timeout; PRAGMA busy_timeout=250; PRAGMA busy_timeout;");
}

#[test]
fn engine_pragma_001_C002() {
    compare_script("engine-pragma", "engine-pragma-001", "C002", "PRAGMA encoding; PRAGMA page_size; PRAGMA journal_mode; PRAGMA synchronous; PRAGMA locking_mode; PRAGMA read_uncommitted;");
}

#[test]
fn engine_pragma_001_C003() {
    compare_script("engine-pragma", "engine-pragma-001", "C003", "PRAGMA trusted_schema; PRAGMA trusted_schema=0; PRAGMA trusted_schema; PRAGMA threads; PRAGMA analysis_limit; PRAGMA reverse_unordered_selects=1; PRAGMA reverse_unordered_selects;");
}
