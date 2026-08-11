//! GENERATED (pack v9): join + scalar-subquery executor replays — no script_table.
mod util;
use util::compare_script;

#[test]
fn engine_subquery_001_C001() {
    compare_script("engine-subquery", "engine-subquery-001", "C001", "CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a, (SELECT max(b) FROM t2) FROM t1 ORDER BY a;");
}

#[test]
fn engine_subquery_001_C002() {
    compare_script("engine-subquery", "engine-subquery-001", "C002", "CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a FROM t1 WHERE a = (SELECT count(*) FROM t2);");
}

#[test]
fn engine_subquery_001_C003() {
    compare_script("engine-subquery", "engine-subquery-001", "C003", "CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT (SELECT b FROM t2 ORDER BY b LIMIT 1), (SELECT b FROM t2 ORDER BY b LIMIT 1 OFFSET 1);");
}

#[test]
fn engine_subquery_001_C004() {
    compare_script("engine-subquery", "engine-subquery-001", "C004", "ATTACH ':memory:' AS aux9; SELECT count(*), (SELECT name FROM pragma_database_list LIMIT 1) FROM pragma_database_list;");
}

#[test]
fn engine_subquery_001_C005() {
    compare_script("engine-subquery", "engine-subquery-001", "C005", "SELECT (SELECT x FROM (SELECT 5 AS x UNION ALL SELECT 3) ORDER BY x LIMIT 1);");
}

#[test]
fn engine_subquery_002_C001() {
    compare_script("engine-subquery", "engine-subquery-002", "C001", "CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT EXISTS(SELECT 1 FROM t2 WHERE b=10), EXISTS(SELECT 1 FROM t2 WHERE b=99);");
}

#[test]
fn engine_subquery_002_C002() {
    compare_script("engine-subquery", "engine-subquery-002", "C002", "CREATE TABLE t1(a INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(b INTEGER); INSERT INTO t2 VALUES(10),(20); SELECT a FROM t1 WHERE NOT EXISTS (SELECT 1 FROM t2 WHERE b = a*10);");
}

#[test]
fn engine_subquery_002_C003() {
    compare_script("engine-subquery", "engine-subquery-002", "C003", "CREATE TABLE t1(x INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(k INTEGER, w INTEGER); INSERT INTO t2 VALUES(1,1),(2,5),(3,3); SELECT x FROM t1 WHERE x = (SELECT w FROM t2 WHERE t2.k = t1.x LIMIT 1);");
}

#[test]
fn engine_subquery_002_C004() {
    compare_script("engine-subquery", "engine-subquery-002", "C004", "CREATE TABLE t1(x INTEGER); INSERT INTO t1 VALUES(1),(2),(3); CREATE TABLE t2(k INTEGER, w INTEGER); INSERT INTO t2 VALUES(1,1),(2,5),(3,3); SELECT x, (SELECT w FROM t2 WHERE t2.k = t1.x) FROM t1 ORDER BY x;");
}

#[test]
fn engine_subquery_002_C005() {
    compare_script("engine-subquery", "engine-subquery-002", "C005", "CREATE TABLE f(v INTEGER); INSERT INTO f VALUES(900017); SELECT (SELECT v FROM f), (SELECT v+1 FROM f), (SELECT v FROM f WHERE v=0);");
}

#[test]
fn engine_join_001_C001() {
    compare_script("engine-join", "engine-join-001", "C001", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, d.dept FROM e JOIN d ON e.id = d.eid ORDER BY e.name, d.dept;");
}

#[test]
fn engine_join_001_C002() {
    compare_script("engine-join", "engine-join-001", "C002", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT count(*) FROM e INNER JOIN d ON e.id = d.eid;");
}

#[test]
fn engine_join_001_C003() {
    compare_script("engine-join", "engine-join-001", "C003", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, d.dept FROM e, d WHERE e.id = d.eid ORDER BY d.dept, e.name;");
}

#[test]
fn engine_join_001_C004() {
    compare_script("engine-join", "engine-join-001", "C004", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT x.name, y.dept FROM e AS x JOIN d AS y ON x.id = y.eid ORDER BY y.dept;");
}

#[test]
fn engine_join_001_C005() {
    compare_script("engine-join", "engine-join-001", "C005", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, d.dept FROM e JOIN d ON e.id = d.eid + 100 ORDER BY e.name;");
}

#[test]
fn engine_join_001_C006() {
    compare_script("engine-join", "engine-join-001", "C006", "CREATE TABLE n(a INTEGER); INSERT INTO n VALUES(1),(2),(3); SELECT p.a, q.a FROM n p JOIN n q ON q.a > p.a ORDER BY p.a, q.a;");
}

#[test]
fn engine_join_001_C007() {
    compare_script("engine-join", "engine-join-001", "C007", "CREATE TABLE ka(k INTEGER); INSERT INTO ka VALUES(910033); CREATE TABLE kb(k INTEGER, t TEXT); INSERT INTO kb VALUES(910033,'hit'),(7,'miss'); SELECT ka.k, kb.t FROM ka JOIN kb ON ka.k = kb.k;");
}

#[test]
fn engine_join_002_C001() {
    compare_script("engine-join", "engine-join-002", "C001", "CREATE TABLE a1(x INTEGER); INSERT INTO a1 VALUES(1),(2); CREATE TABLE b1(x INTEGER, y INTEGER); INSERT INTO b1 VALUES(1,10),(2,20); CREATE TABLE c1(y INTEGER, z TEXT); INSERT INTO c1 VALUES(10,'ten'),(20,'twenty'); SELECT a1.x, c1.z FROM a1 JOIN b1 ON a1.x = b1.x JOIN c1 ON b1.y = c1.y ORDER BY a1.x;");
}

#[test]
fn engine_join_002_C002() {
    compare_script("engine-join", "engine-join-002", "C002", "CREATE TABLE n(a INTEGER); INSERT INTO n VALUES(1),(2),(3); SELECT p.a, q.a FROM n p, n q WHERE q.a = p.a + 1 ORDER BY p.a;");
}

#[test]
fn engine_join_002_C003() {
    compare_script("engine-join", "engine-join-002", "C003", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT count(*), min(e.id), max(d.dept) FROM e JOIN d ON e.id = d.eid;");
}

#[test]
fn engine_join_002_C004() {
    compare_script("engine-join", "engine-join-002", "C004", "CREATE TABLE e(id INTEGER, name TEXT); INSERT INTO e VALUES(1,'ann'),(2,'bob'); CREATE TABLE d(eid INTEGER, dept TEXT); INSERT INTO d VALUES(1,'eng'),(2,'ops'),(1,'qa'); SELECT e.name, count(*) FROM e JOIN d ON e.id = d.eid GROUP BY e.name ORDER BY e.name;");
}

#[test]
fn engine_join_002_C005() {
    compare_script("engine-join", "engine-join-002", "C005", "CREATE TABLE l(a INTEGER); INSERT INTO l VALUES(1),(2),(3); CREATE TABLE r(a INTEGER, t TEXT); INSERT INTO r VALUES(1,'one'),(3,'three'); SELECT l.a, r.t FROM l LEFT JOIN r ON l.a = r.a ORDER BY l.a;");
}

#[test]
fn engine_join_002_C006() {
    compare_script("engine-join", "engine-join-002", "C006", "CREATE TABLE l(a INTEGER); INSERT INTO l VALUES(1),(2),(3); CREATE TABLE r(a INTEGER, t TEXT); INSERT INTO r VALUES(1,'one'),(3,'three'); SELECT count(*), count(r.t) FROM l LEFT JOIN r ON l.a = r.a;");
}

#[test]
fn engine_join_002_C007() {
    compare_script("engine-join", "engine-join-002", "C007", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('b',2),('a',3); SELECT k, sum(v), count(*) FROM g GROUP BY k ORDER BY k;");
}
