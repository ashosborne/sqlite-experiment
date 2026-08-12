//! GENERATED (pack v14): thin-gap-2 executor replays.
mod util;
use util::compare_script;

#[test]
fn engine_window2_001_C001() {
    compare_script("engine-window2", "engine-window2-001", "C001", "CREATE TABLE w(x INTEGER); INSERT INTO w VALUES(10),(20),(20),(30); SELECT x, rank() OVER (ORDER BY x), dense_rank() OVER (ORDER BY x) FROM w ORDER BY x;");
}

#[test]
fn engine_window2_001_C002() {
    compare_script("engine-window2", "engine-window2-001", "C002", "CREATE TABLE v(x INTEGER); INSERT INTO v VALUES(1),(2),(3); SELECT x, lag(x) OVER (ORDER BY x), lead(x) OVER (ORDER BY x) FROM v ORDER BY x;");
}

#[test]
fn engine_window2_001_C003() {
    compare_script("engine-window2", "engine-window2-001", "C003", "CREATE TABLE v(x INTEGER); INSERT INTO v VALUES(10),(20),(30); SELECT x, min(x) OVER (ORDER BY x), max(x) OVER (ORDER BY x), avg(x) OVER (ORDER BY x) FROM v ORDER BY x;");
}

#[test]
fn engine_window2_001_C004() {
    compare_script("engine-window2", "engine-window2-001", "C004", "CREATE TABLE p(g TEXT, x INTEGER); INSERT INTO p VALUES('a',1),('a',2),('b',5),('b',7); SELECT g, x, sum(x) OVER (PARTITION BY g ORDER BY x) FROM p ORDER BY g, x;");
}

#[test]
fn engine_window2_001_C005() {
    compare_script("engine-window2", "engine-window2-001", "C005", "CREATE TABLE w(x INTEGER); INSERT INTO w VALUES(10),(20),(20),(30); SELECT x, sum(x) OVER (ORDER BY x) FROM w ORDER BY x;");
}

#[test]
fn engine_window2_001_C006() {
    compare_script("engine-window2", "engine-window2-001", "C006", "CREATE TABLE w(x INTEGER); INSERT INTO w VALUES(10),(20),(20),(30); SELECT x, sum(x) OVER (ORDER BY x GROUPS BETWEEN 1 PRECEDING AND CURRENT ROW) FROM w ORDER BY x;");
}

#[test]
fn engine_window2_001_C007() {
    compare_script("engine-window2", "engine-window2-001", "C007", "CREATE TABLE w(x INTEGER); INSERT INTO w VALUES(10),(20),(20),(30); SELECT x, count(*) OVER (ORDER BY x ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW), row_number() OVER (ORDER BY x DESC) FROM w ORDER BY x;");
}

#[test]
fn engine_raise_001_C001() {
    compare_script("engine-raise", "engine-raise-001", "C001", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER ti BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(IGNORE); END; INSERT INTO t VALUES(1),(-5),(2); SELECT count(*), sum(a) FROM t;");
}

#[test]
fn engine_raise_001_C002() {
    compare_script("engine-raise", "engine-raise-001", "C002", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tf BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(FAIL,'neg'); END; INSERT INTO t VALUES(-1);");
}

#[test]
fn engine_raise_001_C003() {
    compare_script("engine-raise", "engine-raise-001", "C003", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tr2 BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(ROLLBACK,'neg'); END; INSERT INTO t VALUES(-1);");
}

#[test]
fn engine_raise_001_C004() {
    compare_script("engine-raise", "engine-raise-001", "C004", "CREATE TABLE bt(a INTEGER, b TEXT); INSERT INTO bt VALUES(1,'x'); CREATE VIEW bv AS SELECT a, b FROM bt; CREATE TABLE lg(o INTEGER, n INTEGER); CREATE TRIGGER tu INSTEAD OF UPDATE ON bv BEGIN INSERT INTO lg VALUES(old.a, new.a); END; UPDATE bv SET a = 9 WHERE a = 1; SELECT o, n FROM lg; SELECT a FROM bt;");
}

#[test]
fn engine_raise_001_C005() {
    compare_script("engine-raise", "engine-raise-001", "C005", "CREATE TABLE bt(a INTEGER); INSERT INTO bt VALUES(1),(2); CREATE VIEW bv AS SELECT a FROM bt; CREATE TABLE lg(v INTEGER); CREATE TRIGGER td INSTEAD OF DELETE ON bv BEGIN INSERT INTO lg VALUES(old.a); END; DELETE FROM bv WHERE a = 1; SELECT v FROM lg; SELECT count(*) FROM bt;");
}

#[test]
fn engine_upsert3_001_C001() {
    compare_script("engine-upsert3", "engine-upsert3-001", "C001", "CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER, w TEXT); INSERT INTO u VALUES(1,10,'a'); INSERT INTO u VALUES(1,99,'z') ON CONFLICT(k) DO UPDATE SET v=excluded.v, w=excluded.w; SELECT v, w FROM u;");
}

#[test]
fn engine_upsert3_001_C002() {
    compare_script("engine-upsert3", "engine-upsert3-001", "C002", "CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER, w TEXT); INSERT INTO u VALUES(1,10,'a'); INSERT INTO u VALUES(1,5,'q') ON CONFLICT(k) DO UPDATE SET v = v + excluded.v; SELECT v, w FROM u;");
}

#[test]
fn engine_printf3_001_C001() {
    compare_script("engine-printf3", "engine-printf3-001", "C001", "SELECT printf('%,d', 1234567), printf('%,d', -1234);");
}

#[test]
fn engine_printf3_001_C002() {
    compare_script("engine-printf3", "engine-printf3-001", "C002", "SELECT printf('%p', 123), length(printf('%p', 123)) > 0;");
}

#[test]
fn engine_decimal2_001_C001() {
    compare_script("engine-decimal2", "engine-decimal2-001", "C001", "SELECT decimal_exp('123.5'), decimal_exp('-0.05'), decimal_exp('2');");
}
