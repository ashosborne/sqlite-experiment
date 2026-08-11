//! GENERATED (pack v11): thin-gap-harvest executor replays — no script_table.
mod util;
use util::compare_script;

#[test]
fn engine_agg_having_001_C001() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C001", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT count(v), count(DISTINCT v) FROM g;");
}

#[test]
fn engine_agg_having_001_C002() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C002", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT sum(v), sum(DISTINCT v) FROM g;");
}

#[test]
fn engine_agg_having_001_C003() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C003", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT group_concat(DISTINCT k) FROM g;");
}

#[test]
fn engine_agg_having_001_C004() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C004", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT count(*) FILTER (WHERE v > 1) FROM g;");
}

#[test]
fn engine_agg_having_001_C005() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C005", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT k, sum(v) FROM g GROUP BY k HAVING sum(v) > 4 ORDER BY k;");
}

#[test]
fn engine_agg_having_001_C006() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C006", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT k, count(*) FROM g GROUP BY k HAVING count(*) >= 3 ORDER BY k;");
}

#[test]
fn engine_agg_having_001_C007() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C007", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT k FROM g GROUP BY k HAVING max(v) < 0;");
}

#[test]
fn engine_agg_having_001_C008() {
    compare_script("engine-agg-having", "engine-agg-having-001", "C008", "CREATE TABLE g(k TEXT, v INTEGER); INSERT INTO g VALUES('a',1),('a',1),('a',2),('b',5); SELECT k, total(v) FILTER (WHERE v > 1), count(DISTINCT v) FROM g GROUP BY k HAVING count(*) > 1 ORDER BY k;");
}

#[test]
fn engine_misc2_001_C001() {
    compare_script("engine-misc2", "engine-misc2-001", "C001", "SELECT sha1_query('SELECT 1');");
}

#[test]
fn engine_misc2_001_C002() {
    compare_script("engine-misc2", "engine-misc2-001", "C002", "CREATE TABLE hq(a INTEGER, b TEXT); INSERT INTO hq VALUES(1,'x'),(2,NULL); SELECT sha1_query('SELECT a, b FROM hq');");
}

#[test]
fn engine_misc2_001_C003() {
    compare_script("engine-misc2", "engine-misc2-001", "C003", "SELECT lower(hex(sha3_query('SELECT 1',256)));");
}

#[test]
fn engine_misc2_001_C004() {
    compare_script("engine-misc2", "engine-misc2-001", "C004", "SELECT replace(base85(X'DEADBEEF'),char(10),'|'), is_base85(replace(base85(X'DEADBEEF'),char(10),'')), is_base85('~');");
}

#[test]
fn engine_misc2_001_C005() {
    compare_script("engine-misc2", "engine-misc2-001", "C005", "SELECT hex(base85(base85(X'0102030405')));");
}

#[test]
fn engine_misc2_001_C006() {
    compare_script("engine-misc2", "engine-misc2-001", "C006", "SELECT ieee754_from_blob(X'3FF0000000000000'), hex(ieee754_to_blob(1.5));");
}

#[test]
fn engine_misc2_001_C007() {
    compare_script("engine-misc2", "engine-misc2-001", "C007", "SELECT decimal('7.10'), decimal('-0012.3400'), decimal_pow2(10), decimal_pow2(-3);");
}

#[test]
fn engine_misc2_001_C008() {
    compare_script("engine-misc2", "engine-misc2-001", "C008", "SELECT '10' < '9', '10' < '9' COLLATE decimal;");
}

#[test]
fn engine_misc2_001_C009() {
    compare_script("engine-misc2", "engine-misc2-001", "C009", "CREATE TABLE u(t TEXT); INSERT INTO u VALUES('x10'),('x9'),('x2'); SELECT t FROM u ORDER BY t COLLATE uint;");
}

#[test]
fn engine_misc2_001_C010() {
    compare_script("engine-misc2", "engine-misc2-001", "C010", "SELECT tointeger(X'31') IS NULL, tointeger('9223372036854775808') IS NULL, tointeger(12.0), tointeger(12.5) IS NULL, toreal('abc') IS NULL;");
}

#[test]
fn engine_misc2_001_C011() {
    compare_script("engine-misc2", "engine-misc2-001", "C011", "SELECT uuid_str('{A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11}'), hex(uuid_blob('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'));");
}

#[test]
fn engine_misc2_001_C012() {
    compare_script("engine-misc2", "engine-misc2-001", "C012", "SELECT printf('%w','my\"id'), printf('%#x|%#o', 255, 8);");
}

#[test]
fn engine_order2_001_C001() {
    compare_script("engine-order2", "engine-order2-001", "C001", "CREATE TABLE o(t TEXT); INSERT INTO o VALUES('b'),('A'),('c'),(NULL); SELECT t FROM o ORDER BY t COLLATE nocase NULLS LAST;");
}

#[test]
fn engine_order2_001_C002() {
    compare_script("engine-order2", "engine-order2-001", "C002", "CREATE TABLE o(t TEXT); INSERT INTO o VALUES('b'),('A'),('c'),(NULL); SELECT t FROM o ORDER BY t DESC NULLS FIRST;");
}

#[test]
fn engine_fk2_001_C001() {
    compare_script("engine-fk2", "engine-fk2-001", "C001", "PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id) ON UPDATE CASCADE); INSERT INTO p VALUES(1); INSERT INTO c VALUES(1); UPDATE p SET id=9 WHERE id=1; SELECT pid FROM c;");
}

#[test]
fn engine_fk2_001_C002() {
    compare_script("engine-fk2", "engine-fk2-001", "C002", "PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid REFERENCES p(id) ON UPDATE SET NULL); INSERT INTO p VALUES(1); INSERT INTO c VALUES(1); UPDATE p SET id=9 WHERE id=1; SELECT count(*), count(pid) FROM c;");
}

#[test]
fn engine_fk2_001_C003() {
    compare_script("engine-fk2", "engine-fk2-001", "C003", "PRAGMA foreign_keys=ON; CREATE TABLE p(id INTEGER PRIMARY KEY); CREATE TABLE c(pid INTEGER DEFAULT 7 REFERENCES p(id) ON DELETE SET DEFAULT); INSERT INTO p VALUES(1),(7); INSERT INTO c VALUES(1); DELETE FROM p WHERE id=1; SELECT pid FROM c;");
}

#[test]
fn engine_trig2_001_C001() {
    compare_script("engine-trig2", "engine-trig2-001", "C001", "CREATE TABLE bt(a INTEGER); CREATE VIEW bv AS SELECT a FROM bt; CREATE TRIGGER ti INSTEAD OF INSERT ON bv BEGIN INSERT INTO bt VALUES(new.a * 10); END; INSERT INTO bv VALUES(4); SELECT a FROM bt;");
}

#[test]
fn engine_trig2_001_C002() {
    compare_script("engine-trig2", "engine-trig2-001", "C002", "CREATE TABLE t(a INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tg AFTER INSERT ON t BEGIN INSERT INTO lg VALUES(1); END; SELECT count(*) FROM sqlite_master WHERE type='trigger'; DROP TRIGGER tg; SELECT count(*) FROM sqlite_master WHERE type='trigger'; INSERT INTO t VALUES(1); SELECT count(*) FROM lg;");
}

#[test]
fn engine_trig2_001_C003() {
    compare_script("engine-trig2", "engine-trig2-001", "C003", "CREATE TABLE t(a INTEGER); CREATE TRIGGER tr BEFORE INSERT ON t WHEN new.a < 0 BEGIN SELECT RAISE(ABORT,'no negatives'); END; INSERT INTO t VALUES(-5);");
}

#[test]
fn engine_trig2_001_C004() {
    compare_script("engine-trig2", "engine-trig2-001", "C004", "CREATE TABLE t(a INTEGER, b INTEGER); CREATE TABLE lg(v INTEGER); CREATE TRIGGER tu AFTER UPDATE OF a ON t BEGIN INSERT INTO lg VALUES(1); END; INSERT INTO t VALUES(1,1); UPDATE t SET b=2 WHERE a=1; UPDATE t SET a=3 WHERE b=2; SELECT count(*) FROM lg;");
}

#[test]
fn engine_trig2_001_C005() {
    compare_script("engine-trig2", "engine-trig2-001", "C005", "PRAGMA recursive_triggers=ON; CREATE TABLE t(a INTEGER); CREATE TRIGGER tr AFTER INSERT ON t WHEN new.a < 3 BEGIN INSERT INTO t VALUES(new.a + 1); END; INSERT INTO t VALUES(1); SELECT count(*), max(a) FROM t;");
}

#[test]
fn engine_ddl2_001_C001() {
    compare_script("engine-ddl2", "engine-ddl2-001", "C001", "CREATE TABLE r(a INTEGER, b TEXT); INSERT INTO r VALUES(1,'x'); ALTER TABLE r RENAME COLUMN b TO c; SELECT c FROM r;");
}

#[test]
fn engine_ddl2_001_C002() {
    compare_script("engine-ddl2", "engine-ddl2-001", "C002", "CREATE TABLE r(a INTEGER, b TEXT); INSERT INTO r VALUES(1,'x'); ALTER TABLE r DROP COLUMN b; SELECT count(*) FROM pragma_table_info('r'); SELECT a FROM r;");
}

#[test]
fn engine_upsert2_001_C001() {
    compare_script("engine-upsert2", "engine-upsert2-001", "C001", "CREATE TABLE u(k INTEGER PRIMARY KEY, v INTEGER); INSERT INTO u VALUES(1,10); INSERT INTO u VALUES(1,99) ON CONFLICT(k) DO UPDATE SET v = excluded.v WHERE excluded.v > 50; INSERT INTO u VALUES(1,5) ON CONFLICT(k) DO UPDATE SET v = excluded.v WHERE excluded.v > 50; SELECT v FROM u;");
}
