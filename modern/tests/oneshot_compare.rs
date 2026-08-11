//! GENERATED run-11 compare test: drives the Rust exec against every frozen
//! script golden (read-only) and asserts the identical OBS derivation.
//! Not factory COMPARE — parity_green untouched.
mod util;
use util::compare_script;

#[test]
fn pragma_surface_001_c001() {
    compare_script("pragma-surface", "pragma-surface-001", "C001", "PRAGMA user_version; PRAGMA user_version=7; PRAGMA user_version;");
}

#[test]
fn pragma_surface_002_c001() {
    compare_script("pragma-surface", "pragma-surface-002", "C001", "SELECT count(*), (SELECT name FROM pragma_database_list LIMIT 1) FROM pragma_database_list;");
}

#[test]
fn attach_detach_001_c001() {
    compare_script("attach-detach", "attach-detach-001", "C001", "ATTACH ':memory:' AS aux1; SELECT count(*) FROM pragma_database_list;");
}

#[test]
fn attach_detach_002_c001() {
    compare_script("attach-detach", "attach-detach-002", "C001", "ATTACH ':memory:' AS aux2; DETACH aux2; SELECT count(*) FROM pragma_database_list;");
}

#[test]
fn printf_format_001_c001() {
    compare_script("printf-format", "printf-format-001", "C001", "SELECT printf('%d|%s|%q', 7, 'a', 'a''b');");
}

#[test]
fn json_funcs_001_c001() {
    compare_script("json-funcs", "json-funcs-001", "C001", "SELECT json_extract('{\"a\":{\"b\":2}}','$.a.b'), '{\"a\":1}' -> '$.a', '{\"a\":1}' ->> '$.a';");
}

#[test]
fn json_funcs_002_c001() {
    compare_script("json-funcs", "json-funcs-002", "C001", "SELECT json_set('{}','$.a',1), json_remove('{\"a\":1,\"b\":2}','$.b'), json_patch('{\"a\":1}','{\"b\":2}');");
}

#[test]
fn json_funcs_003_c001() {
    compare_script("json-funcs", "json-funcs-003", "C001", "SELECT json_valid('{}'), json_valid('{'), json_type('[3]','$[0]');");
}

#[test]
fn json_funcs_004_c001() {
    compare_script("json-funcs", "json-funcs-004", "C001", "SELECT count(*), sum(value) FROM json_each('[3,4,5]');");
}

#[test]
fn date_time_funcs_001_c001() {
    compare_script("date-time-funcs", "date-time-funcs-001", "C001", "SELECT date('2026-08-11'), datetime(2460000.5), unixepoch('2001-01-01');");
}

#[test]
fn date_time_funcs_002_c001() {
    compare_script("date-time-funcs", "date-time-funcs-002", "C001", "SELECT strftime('%Y|%m|%d|%j','2026-08-11');");
}

#[test]
fn date_time_funcs_003_c001() {
    compare_script("date-time-funcs", "date-time-funcs-003", "C001", "SELECT datetime('2026-01-31','+1 month'), date('2026-08-11','weekday 0');");
}

#[test]
fn date_time_funcs_004_c001() {
    compare_script("date-time-funcs", "date-time-funcs-004", "C001", "SELECT timediff('2026-08-11','2025-08-10');");
}

#[test]
fn builtin_scalar_agg_funcs_001_c001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-001", "C001", "SELECT upper('abc'), length('hello'), substr('abcdef',-3,2), coalesce(NULL,7), typeof(2.0);");
}

#[test]
fn builtin_scalar_agg_funcs_002_c001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-002", "C001", "SELECT sum(x), total(x), count(*), count(x), group_concat(x,'-') FROM (SELECT 1 AS x UNION ALL SELECT NULL UNION ALL SELECT 2);");
}

#[test]
fn builtin_scalar_agg_funcs_003_c001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-003", "C001", "SELECT 'abc' LIKE 'A%', 'abc' GLOB 'A*', 'a%c' LIKE 'a\\%c' ESCAPE '\\';");
}

#[test]
fn ddl_schema_001_c001() {
    compare_script("ddl-schema", "ddl-schema-001", "C001", "CREATE TABLE t1(a INTEGER PRIMARY KEY, b TEXT); SELECT count(*) FROM sqlite_master WHERE name='t1'; DROP TABLE t1; SELECT count(*) FROM sqlite_master;");
}

#[test]
fn ddl_schema_002_c001() {
    compare_script("ddl-schema", "ddl-schema-002", "C001", "CREATE TABLE t2(a); CREATE UNIQUE INDEX i2 ON t2(a); INSERT INTO t2 VALUES(1); INSERT OR IGNORE INTO t2 VALUES(1); SELECT count(*) FROM t2;");
}

#[test]
fn ddl_schema_003_c001() {
    compare_script("ddl-schema", "ddl-schema-003", "C001", "CREATE TABLE t3(a); ALTER TABLE t3 RENAME TO t3x; ALTER TABLE t3x ADD COLUMN b DEFAULT 5; INSERT INTO t3x(a) VALUES(9); SELECT a,b FROM t3x;");
}

#[test]
fn dml_codegen_001_c001() {
    compare_script("dml-codegen", "dml-codegen-001", "C001", "CREATE TABLE d(a); INSERT INTO d VALUES(1),(2); UPDATE d SET a=a+10 WHERE a=2; DELETE FROM d WHERE a=1; SELECT a, changes(), total_changes() FROM d;");
}

#[test]
fn dml_codegen_002_c001() {
    compare_script("dml-codegen", "dml-codegen-002", "C001", "CREATE TABLE u(a UNIQUE); INSERT INTO u VALUES(1); INSERT OR REPLACE INTO u VALUES(1); INSERT OR IGNORE INTO u VALUES(1); SELECT count(*) FROM u;");
}

#[test]
fn expr_codegen_001_c001() {
    compare_script("expr-codegen", "expr-codegen-001", "C001", "SELECT 1+2*3, 'a'||'b', CAST('12x' AS INTEGER), CAST(2.9 AS INTEGER);");
}

#[test]
fn expr_codegen_002_c001() {
    compare_script("expr-codegen", "expr-codegen-002", "C001", "SELECT 1 IN (2,NULL), 3 IN (2,NULL,3), NULL IS NULL, NULL = NULL;");
}

#[test]
fn expr_codegen_003_c001() {
    compare_script("expr-codegen", "expr-codegen-003", "C001", "SELECT CASE WHEN 1 THEN 'y' ELSE 'n' END, iif(0,'a','b');");
}

#[test]
fn select_codegen_001_c001() {
    compare_script("select-codegen", "select-codegen-001", "C001", "SELECT a FROM (SELECT 2 AS a UNION ALL SELECT 1) ORDER BY a;");
}

#[test]
fn select_codegen_002_c001() {
    compare_script("select-codegen", "select-codegen-002", "C001", "SELECT 2 UNION SELECT 1 ORDER BY 1;");
}

#[test]
fn select_codegen_003_c001() {
    compare_script("select-codegen", "select-codegen-003", "C001", "SELECT x FROM (SELECT 1 AS x) WHERE x=1;");
}

#[test]
fn name_resolution_001_c001() {
    compare_script("name-resolution", "name-resolution-001", "C001", "CREATE TABLE n1(a); INSERT INTO n1 VALUES(5); SELECT n1.a, a, rowid FROM n1;");
}

#[test]
fn name_resolution_002_c001() {
    compare_script("name-resolution", "name-resolution-002", "C001", "SELECT 3 AS k UNION ALL SELECT 1 ORDER BY k;");
}

#[test]
fn tokenizer_001_c001() {
    compare_script("tokenizer", "tokenizer-001", "C001", "SELECT 0x10, 1e2, X'41', [a] FROM (SELECT 1 AS a);");
}

#[test]
fn parser_grammar_001_c001() {
    compare_script("parser-grammar", "parser-grammar-001", "C001", "CREATE TABLE p1(key TEXT, abort INT); INSERT INTO p1 VALUES('k',1); SELECT key, abort FROM p1;");
}

#[test]
fn parser_grammar_002_c001() {
    compare_script("parser-grammar", "parser-grammar-002", "C001", "SELECT FROM;");
}

#[test]
fn analyze_stats_001_c001() {
    compare_script("analyze-stats", "analyze-stats-001", "C001", "CREATE TABLE s1(a); INSERT INTO s1 VALUES(1),(2); CREATE INDEX si ON s1(a); ANALYZE; SELECT count(*) FROM sqlite_master WHERE name='sqlite_stat1'; SELECT stat FROM sqlite_stat1 WHERE idx='si';");
}

#[test]
fn foreign_keys_001_c001() {
    compare_script("foreign-keys", "foreign-keys-001", "C001", "PRAGMA foreign_keys=ON; CREATE TABLE par(id INTEGER PRIMARY KEY); CREATE TABLE chi(pid REFERENCES par(id)); INSERT INTO chi VALUES(1);");
}

#[test]
fn foreign_keys_002_c001() {
    compare_script("foreign-keys", "foreign-keys-002", "C001", "PRAGMA foreign_keys=ON; CREATE TABLE p2(id INTEGER PRIMARY KEY); CREATE TABLE c2(pid REFERENCES p2(id) ON DELETE CASCADE); INSERT INTO p2 VALUES(1); INSERT INTO c2 VALUES(1); DELETE FROM p2; SELECT count(*) FROM c2;");
}

#[test]
fn foreign_keys_003_c001() {
    compare_script("foreign-keys", "foreign-keys-003", "C001", "PRAGMA foreign_keys=ON; CREATE TABLE p3(id INTEGER PRIMARY KEY); CREATE TABLE c3(pid REFERENCES p3(id)); INSERT INTO p3 VALUES(1); INSERT INTO c3 VALUES(1); DROP TABLE p3;");
}

#[test]
fn triggers_001_c001() {
    compare_script("triggers", "triggers-001", "C001", "CREATE TABLE tr(a); CREATE TABLE tlog(v); CREATE TRIGGER trg AFTER INSERT ON tr BEGIN INSERT INTO tlog VALUES(new.a); END; SELECT count(*) FROM sqlite_master WHERE type='trigger';");
}

#[test]
fn triggers_002_c001() {
    compare_script("triggers", "triggers-002", "C001", "CREATE TABLE tr2(a); CREATE TABLE tlog2(v); CREATE TRIGGER trg2 AFTER INSERT ON tr2 BEGIN INSERT INTO tlog2 VALUES(new.a*2); END; INSERT INTO tr2 VALUES(7); SELECT v FROM tlog2;");
}

#[test]
fn upsert_001_c001() {
    compare_script("upsert", "upsert-001", "C001", "CREATE TABLE up(a INTEGER PRIMARY KEY, b); INSERT INTO up VALUES(1,'x'); INSERT INTO up VALUES(1,'y') ON CONFLICT(a) DO NOTHING; SELECT b, count(*) FROM up;");
}

#[test]
fn upsert_002_c001() {
    compare_script("upsert", "upsert-002", "C001", "CREATE TABLE up2(a INTEGER PRIMARY KEY, b); INSERT INTO up2 VALUES(1,'x'); INSERT INTO up2 VALUES(1,'y') ON CONFLICT(a) DO UPDATE SET b=excluded.b; SELECT b FROM up2;");
}

#[test]
fn vacuum_001_c001() {
    compare_script("vacuum", "vacuum-001", "C001", "CREATE TABLE v1(a); INSERT INTO v1 VALUES(zeroblob(1000)); DROP TABLE v1; VACUUM; SELECT 1;");
}

#[test]
fn window_functions_001_c001() {
    compare_script("window-functions", "window-functions-001", "C001", "SELECT x, row_number() OVER (ORDER BY x) FROM (SELECT 30 AS x UNION ALL SELECT 10 UNION ALL SELECT 20) ORDER BY x;");
}

#[test]
fn window_functions_002_c001() {
    compare_script("window-functions", "window-functions-002", "C001", "SELECT x, sum(x) OVER (ORDER BY x ROWS BETWEEN 1 PRECEDING AND CURRENT ROW) FROM (SELECT 10 AS x UNION ALL SELECT 20 UNION ALL SELECT 30) ORDER BY x;");
}

#[test]
fn introspection_vtabs_001_c001() {
    compare_script("introspection-vtabs", "introspection-vtabs-001", "C001", "CREATE TABLE iv(a); INSERT INTO iv VALUES(1); SELECT count(*)>0 FROM dbstat;");
}

#[test]
fn misc_uuid_001_c001() {
    compare_script("misc-uuid", "misc-uuid-001", "C001", "SELECT length(uuid()), substr(uuid(),15,1), uuid_str(uuid_blob(uuid())) IS NOT NULL;");
}

#[test]
fn misc_regexp_001_c001() {
    compare_script("misc-regexp", "misc-regexp-001", "C001", "SELECT 'abc' REGEXP 'a.c', 'abc' REGEXP '^b';");
}

#[test]
fn misc_series_001_c001() {
    compare_script("misc-series", "misc-series-001", "C001", "SELECT count(*), sum(value) FROM generate_series(1,5);");
}

#[test]
fn misc_csv_001_c001() {
    compare_script("misc-csv", "misc-csv-001", "C001", "CREATE VIRTUAL TABLE temp.c1 USING csv(data='1,2\n3,4'); SELECT c0,c1 FROM c1;");
}

#[test]
fn misc_decimal_001_c001() {
    compare_script("misc-decimal", "misc-decimal-001", "C001", "SELECT decimal_add('1.10','2.25'), decimal_cmp('2','10');");
}

#[test]
fn misc_basexx_001_c001() {
    compare_script("misc-basexx", "misc-basexx-001", "C001", "SELECT base64(X'01FF'), hex(base64('Af8='));");
}

#[test]
fn misc_rot13_001_c001() {
    compare_script("misc-rot13", "misc-rot13-001", "C001", "SELECT rot13('Hello'), rot13(rot13('Hello'));");
}

#[test]
fn misc_totype_001_c001() {
    compare_script("misc-totype", "misc-totype-001", "C001", "SELECT tointeger('42'), tointeger('1.5') IS NULL, toreal('2.5');");
}

#[test]
fn misc_uint_001_c001() {
    compare_script("misc-uint", "misc-uint-001", "C001", "SELECT 'x9' < 'x10' COLLATE uint, 'x9' < 'x10';");
}

#[test]
fn misc_ieee754_001_c001() {
    compare_script("misc-ieee754", "misc-ieee754-001", "C001", "SELECT ieee754(2.5), ieee754_mantissa(2.5), ieee754_exponent(2.5);");
}

#[test]
fn misc_percentile_001_c001() {
    compare_script("misc-percentile", "misc-percentile-001", "C001", "SELECT median(x), percentile(x,25) FROM (SELECT 1 AS x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4);");
}

#[test]
fn misc_completion_001_c001() {
    compare_script("misc-completion", "misc-completion-001", "C001", "SELECT count(*)>0 FROM completion('SEL');");
}

#[test]
fn misc_prefixes_001_c001() {
    compare_script("misc-prefixes", "misc-prefixes-001", "C001", "SELECT count(*) FROM prefixes('abc');");
}

#[test]
fn misc_wholenumber_001_c001() {
    compare_script("misc-wholenumber", "misc-wholenumber-001", "C001", "CREATE VIRTUAL TABLE temp.w USING wholenumber; SELECT count(*) FROM w WHERE value BETWEEN 1 AND 5;");
}

#[test]
fn misc_compress_001_c001() {
    compare_script("misc-compress", "misc-compress-001", "C001", "SELECT hex(uncompress(compress('hello')))=hex('hello'), length(compress(zeroblob(1000)))<1000;");
}

#[test]
fn misc_fossildelta_001_c001() {
    compare_script("misc-fossildelta", "misc-fossildelta-001", "C001", "SELECT delta_apply('abc', delta_create('abc','abcd'))='abcd', delta_output_size(delta_create('abc','abcd'));");
}

#[test]
fn misc_nextchar_001_c001() {
    compare_script("misc-nextchar", "misc-nextchar-001", "C001", "CREATE TABLE w(x TEXT); CREATE INDEX wx ON w(x); INSERT INTO w VALUES('cat'),('car'),('cow'); SELECT next_char('ca','w','x');");
}

#[test]
fn misc_sha1_001_c001() {
    compare_script("misc-sha1", "misc-sha1-001", "C001", "SELECT sha1('abc');");
}

#[test]
fn misc_shathree_001_c001() {
    compare_script("misc-shathree", "misc-shathree-001", "C001", "SELECT lower(hex(sha3('abc',256)));");
}

#[test]
fn misc_urifuncs_001_c001() {
    compare_script("misc-urifuncs", "misc-urifuncs-001", "C001", "SELECT sqlite3_uri_parameter('main','vfs') IS NULL, sqlite3_uri_boolean('main','ro',0);");
}

#[test]
fn misc_utilities_001_c001() {
    compare_script("misc-utilities", "misc-utilities-001", "C001", "SELECT eval('SELECT 3'), eval('SELECT 1; SELECT 2');");
}

#[test]
fn misc_zorder_001_c001() {
    compare_script("misc-zorder", "misc-zorder-001", "C001", "SELECT zorder(3,5), unzorder(zorder(3,5),2,0), unzorder(zorder(3,5),2,1);");
}

#[test]
fn misc_func_packs_001_c001() {
    compare_script("misc-func-packs", "misc-func-packs-001", "C001", "SELECT decimal_mul('1.5','2'), 'pack' REGEXP 'p.ck';");
}

#[test]
fn pragma_surface_001_c002() {
    compare_script("pragma-surface", "pragma-surface-001", "C002", "PRAGMA application_id; PRAGMA application_id=42; PRAGMA application_id;");
}

#[test]
fn pragma_surface_001_c003() {
    compare_script("pragma-surface", "pragma-surface-001", "C003", "PRAGMA schema_version; CREATE TABLE t(a); PRAGMA schema_version;");
}

#[test]
fn pragma_surface_001_c004() {
    compare_script("pragma-surface", "pragma-surface-001", "C004", "PRAGMA cache_size; PRAGMA cache_size=100; PRAGMA cache_size;");
}

#[test]
fn pragma_surface_001_c005() {
    compare_script("pragma-surface", "pragma-surface-001", "C005", "PRAGMA recursive_triggers; PRAGMA recursive_triggers=ON; PRAGMA recursive_triggers;");
}

#[test]
fn pragma_surface_001_c006() {
    compare_script("pragma-surface", "pragma-surface-001", "C006", "PRAGMA defer_foreign_keys; PRAGMA defer_foreign_keys=1; PRAGMA defer_foreign_keys;");
}

#[test]
fn pragma_surface_001_c007() {
    compare_script("pragma-surface", "pragma-surface-001", "C007", "PRAGMA query_only; PRAGMA query_only=1; PRAGMA query_only;");
}

#[test]
fn pragma_surface_001_c008() {
    compare_script("pragma-surface", "pragma-surface-001", "C008", "PRAGMA temp_store; PRAGMA temp_store=2; PRAGMA temp_store;");
}

#[test]
fn pragma_surface_001_c009() {
    compare_script("pragma-surface", "pragma-surface-001", "C009", "PRAGMA automatic_index; PRAGMA automatic_index=0; PRAGMA automatic_index;");
}

#[test]
fn pragma_surface_001_c010() {
    compare_script("pragma-surface", "pragma-surface-001", "C010", "PRAGMA ignore_check_constraints; PRAGMA ignore_check_constraints=1; PRAGMA ignore_check_constraints;");
}

#[test]
fn pragma_surface_001_c011() {
    compare_script("pragma-surface", "pragma-surface-001", "C011", "PRAGMA case_sensitive_like=ON; SELECT 'abc' LIKE 'A%'; PRAGMA case_sensitive_like=OFF; SELECT 'abc' LIKE 'A%';");
}

#[test]
fn pragma_surface_001_c012() {
    compare_script("pragma-surface", "pragma-surface-001", "C012", "CREATE TABLE ic(a CHECK(a>0)); PRAGMA integrity_check; PRAGMA quick_check;");
}

#[test]
fn pragma_surface_002_c002() {
    compare_script("pragma-surface", "pragma-surface-002", "C002", "CREATE TABLE pt(a INTEGER PRIMARY KEY, b TEXT REFERENCES pt(a)); CREATE INDEX pi ON pt(b); SELECT count(*) FROM pragma_table_info('pt'); SELECT count(*) FROM pragma_foreign_key_list('pt'); SELECT count(*) FROM pragma_index_list('pt');");
}

#[test]
fn pragma_surface_002_c003() {
    compare_script("pragma-surface", "pragma-surface-002", "C003", "SELECT count(*) FROM pragma_compile_options;");
}

#[test]
fn pragma_surface_002_c004() {
    compare_script("pragma-surface", "pragma-surface-002", "C004", "SELECT count(*) FROM pragma_function_list, (SELECT 1) WHERE (SELECT count(*) FROM pragma_module_list)=18 AND (SELECT count(*) FROM pragma_pragma_list)=66;");
}

#[test]
fn attach_detach_003_c001() {
    compare_script("attach-detach", "attach-detach-003", "C001", "ATTACH ':memory:' AS aux3; CREATE TABLE main.mm(v); CREATE TABLE aux3.t(a); CREATE TRIGGER aux3.trg AFTER INSERT ON aux3.t BEGIN INSERT INTO main.mm VALUES(1); END;");
}
