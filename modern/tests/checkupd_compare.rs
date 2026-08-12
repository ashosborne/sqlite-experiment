//! GENERATED (pack v16): CHECK-on-UPDATE executor replays.
mod util;
use util::compare_script;

#[test]
fn engine_checkupd_001_C001() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C001", "CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); UPDATE c SET a = -1;");
}

#[test]
fn engine_checkupd_001_C002() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C002", "CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); UPDATE c SET a = 7; SELECT a FROM c;");
}

#[test]
fn engine_checkupd_001_C003() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C003", "CREATE TABLE m(a INTEGER, b INTEGER, CHECK(a < b)); INSERT INTO m VALUES(1,5); UPDATE m SET a = 3; SELECT a, b FROM m;");
}

#[test]
fn engine_checkupd_001_C004() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C004", "CREATE TABLE m(a INTEGER, b INTEGER, CHECK(a < b)); INSERT INTO m VALUES(1,5); UPDATE m SET a = 9;");
}

#[test]
fn engine_checkupd_001_C005() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C005", "CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); INSERT INTO c VALUES(20); UPDATE OR IGNORE c SET a = -1 WHERE a = 5; UPDATE OR IGNORE c SET a = 15 WHERE a = 20; SELECT a FROM c ORDER BY a;");
}

#[test]
fn engine_checkupd_001_C006() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C006", "CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); UPDATE c SET a = NULL; SELECT count(*), count(a) FROM c;");
}

#[test]
fn engine_checkupd_001_C007() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C007", "CREATE TABLE e(a INTEGER CHECK(a % 2 = 0)); INSERT INTO e VALUES(2); UPDATE e SET a = 4; SELECT a FROM e;");
}

#[test]
fn engine_checkupd_001_C008() {
    compare_script("engine-checkupd", "engine-checkupd-001", "C008", "CREATE TABLE c(a INTEGER CHECK(a > 0)); INSERT INTO c VALUES(5); BEGIN; UPDATE c SET a = 9; COMMIT; SELECT a FROM c;");
}
