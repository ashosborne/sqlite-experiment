//! GENERATED (pack v15): transaction/savepoint executor replays.
mod util;
use util::compare_script;

#[test]
fn engine_txn_001_C001() {
    compare_script("engine-txn", "engine-txn-001", "C001", "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); COMMIT; SELECT a FROM t;");
}

#[test]
fn engine_txn_001_C002() {
    compare_script("engine-txn", "engine-txn-001", "C002", "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); ROLLBACK; SELECT count(*) FROM t;");
}

#[test]
fn engine_txn_001_C003() {
    compare_script("engine-txn", "engine-txn-001", "C003", "BEGIN; BEGIN;");
}

#[test]
fn engine_txn_001_C004() {
    compare_script("engine-txn", "engine-txn-001", "C004", "COMMIT;");
}

#[test]
fn engine_txn_001_C005() {
    compare_script("engine-txn", "engine-txn-001", "C005", "ROLLBACK;");
}

#[test]
fn engine_txn_001_C006() {
    compare_script("engine-txn", "engine-txn-001", "C006", "CREATE TABLE t(a INTEGER); BEGIN; INSERT INTO t VALUES(1); INSERT INTO t VALUES(2); INSERT INTO t VALUES(3); ROLLBACK; SELECT count(*) FROM t;");
}

#[test]
fn engine_txn_001_C007() {
    compare_script("engine-txn", "engine-txn-001", "C007", "CREATE TABLE t(a INTEGER); BEGIN; CREATE TABLE d(x INTEGER); INSERT INTO d VALUES(5); ROLLBACK; SELECT count(*) FROM sqlite_master WHERE name='d'; SELECT count(*) FROM t;");
}

#[test]
fn engine_txn_001_C008() {
    compare_script("engine-txn", "engine-txn-001", "C008", "CREATE TABLE t(a INTEGER); BEGIN DEFERRED; INSERT INTO t VALUES(4); COMMIT; BEGIN IMMEDIATE; INSERT INTO t VALUES(5); COMMIT; SELECT count(*) FROM t;");
}

#[test]
fn engine_savepoint_001_C001() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C001", "CREATE TABLE q(a INTEGER); SAVEPOINT s1; INSERT INTO q VALUES(1); ROLLBACK TO s1; RELEASE s1; SELECT count(*) FROM q;");
}

#[test]
fn engine_savepoint_001_C002() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C002", "CREATE TABLE q(a INTEGER); INSERT INTO q VALUES(0); SAVEPOINT s1; INSERT INTO q VALUES(1); RELEASE s1; SELECT count(*) FROM q;");
}

#[test]
fn engine_savepoint_001_C003() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C003", "CREATE TABLE q(a INTEGER); SAVEPOINT a; INSERT INTO q VALUES(1); SAVEPOINT b; INSERT INTO q VALUES(2); ROLLBACK TO b; RELEASE a; SELECT count(*) FROM q;");
}

#[test]
fn engine_savepoint_001_C004() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C004", "CREATE TABLE q(a INTEGER); SAVEPOINT a; INSERT INTO q VALUES(1); ROLLBACK TO a; INSERT INTO q VALUES(2); RELEASE a; SELECT a FROM q;");
}

#[test]
fn engine_savepoint_001_C005() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C005", "RELEASE nosp;");
}

#[test]
fn engine_savepoint_001_C006() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C006", "CREATE TABLE q(a INTEGER); SAVEPOINT a; INSERT INTO q VALUES(1); SAVEPOINT b; INSERT INTO q VALUES(2); ROLLBACK TO a; RELEASE a; SELECT count(*) FROM q;");
}

#[test]
fn engine_savepoint_001_C007() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C007", "CREATE TABLE q(a INTEGER); BEGIN; INSERT INTO q VALUES(1); SAVEPOINT s; INSERT INTO q VALUES(2); ROLLBACK TO s; COMMIT; SELECT a FROM q;");
}

#[test]
fn engine_savepoint_001_C008() {
    compare_script("engine-savepoint", "engine-savepoint-001", "C008", "CREATE TABLE q(a INTEGER); SAVEPOINT a; RELEASE a; ROLLBACK TO a;");
}

#[test]
fn engine_txn_002_C001() {
    compare_script("engine-txn", "engine-txn-002", "C001", "CREATE TABLE u(a INTEGER UNIQUE); BEGIN; INSERT INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1); COMMIT; SELECT count(*) FROM u;");
}
