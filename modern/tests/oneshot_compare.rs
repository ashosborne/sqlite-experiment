//! GENERATED (pack v8): store+eval executor replays — no script_table.
mod util;
use util::compare_script;

#[test]
fn pragma_surface_001_C001() {
    compare_script("pragma-surface", "pragma-surface-001", "C001", "PRAGMA user_version; PRAGMA user_version=7; PRAGMA user_version;");
}

#[test]
fn pragma_surface_002_C001() {
    compare_script("pragma-surface", "pragma-surface-002", "C001", "SELECT count(*), (SELECT name FROM pragma_database_list LIMIT 1) FROM pragma_database_list;");
}

#[test]
fn attach_detach_001_C001() {
    compare_script("attach-detach", "attach-detach-001", "C001", "ATTACH ':memory:' AS aux1; SELECT count(*) FROM pragma_database_list;");
}

#[test]
fn attach_detach_002_C001() {
    compare_script("attach-detach", "attach-detach-002", "C001", "ATTACH ':memory:' AS aux2; DETACH aux2; SELECT count(*) FROM pragma_database_list;");
}

#[test]
fn printf_format_001_C001() {
    compare_script("printf-format", "printf-format-001", "C001", "SELECT printf('%d|%s|%q', 7, 'a', 'a''b');");
}

#[test]
fn json_funcs_001_C001() {
    compare_script("json-funcs", "json-funcs-001", "C001", "SELECT json_extract('{\"a\":{\"b\":2}}','$.a.b'), '{\"a\":1}' -> '$.a', '{\"a\":1}' ->> '$.a';");
}

#[test]
fn json_funcs_002_C001() {
    compare_script("json-funcs", "json-funcs-002", "C001", "SELECT json_set('{}','$.a',1), json_remove('{\"a\":1,\"b\":2}','$.b'), json_patch('{\"a\":1}','{\"b\":2}');");
}

#[test]
fn json_funcs_003_C001() {
    compare_script("json-funcs", "json-funcs-003", "C001", "SELECT json_valid('{}'), json_valid('{'), json_type('[3]','$[0]');");
}

#[test]
fn json_funcs_004_C001() {
    compare_script("json-funcs", "json-funcs-004", "C001", "SELECT count(*), sum(value) FROM json_each('[3,4,5]');");
}

#[test]
fn builtin_scalar_agg_funcs_001_C001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-001", "C001", "SELECT upper('abc'), length('hello'), substr('abcdef',-3,2), coalesce(NULL,7), typeof(2.0);");
}

#[test]
fn builtin_scalar_agg_funcs_002_C001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-002", "C001", "SELECT sum(x), total(x), count(*), count(x), group_concat(x,'-') FROM (SELECT 1 AS x UNION ALL SELECT NULL UNION ALL SELECT 2);");
}

#[test]
fn builtin_scalar_agg_funcs_003_C001() {
    compare_script("builtin-scalar-agg-funcs", "builtin-scalar-agg-funcs-003", "C001", "SELECT 'abc' LIKE 'A%', 'abc' GLOB 'A*', 'a%c' LIKE 'a\\%c' ESCAPE '\\';");
}

#[test]
fn expr_codegen_001_C001() {
    compare_script("expr-codegen", "expr-codegen-001", "C001", "SELECT 1+2*3, 'a'||'b', CAST('12x' AS INTEGER), CAST(2.9 AS INTEGER);");
}

#[test]
fn expr_codegen_002_C001() {
    compare_script("expr-codegen", "expr-codegen-002", "C001", "SELECT 1 IN (2,NULL), 3 IN (2,NULL,3), NULL IS NULL, NULL = NULL;");
}

#[test]
fn expr_codegen_003_C001() {
    compare_script("expr-codegen", "expr-codegen-003", "C001", "SELECT CASE WHEN 1 THEN 'y' ELSE 'n' END, iif(0,'a','b');");
}

#[test]
fn select_codegen_001_C001() {
    compare_script("select-codegen", "select-codegen-001", "C001", "SELECT a FROM (SELECT 2 AS a UNION ALL SELECT 1) ORDER BY a;");
}

#[test]
fn select_codegen_002_C001() {
    compare_script("select-codegen", "select-codegen-002", "C001", "SELECT 2 UNION SELECT 1 ORDER BY 1;");
}

#[test]
fn select_codegen_003_C001() {
    compare_script("select-codegen", "select-codegen-003", "C001", "SELECT x FROM (SELECT 1 AS x) WHERE x=1;");
}

#[test]
fn name_resolution_002_C001() {
    compare_script("name-resolution", "name-resolution-002", "C001", "SELECT 3 AS k UNION ALL SELECT 1 ORDER BY k;");
}

#[test]
fn tokenizer_001_C001() {
    compare_script("tokenizer", "tokenizer-001", "C001", "SELECT 0x10, 1e2, X'41', [a] FROM (SELECT 1 AS a);");
}

#[test]
fn parser_grammar_001_C001() {
    compare_script("parser-grammar", "parser-grammar-001", "C001", "CREATE TABLE p1(key TEXT, abort INT); INSERT INTO p1 VALUES('k',1); SELECT key, abort FROM p1;");
}

#[test]
fn parser_grammar_002_C001() {
    compare_script("parser-grammar", "parser-grammar-002", "C001", "SELECT FROM;");
}

#[test]
fn window_functions_001_C001() {
    compare_script("window-functions", "window-functions-001", "C001", "SELECT x, row_number() OVER (ORDER BY x) FROM (SELECT 30 AS x UNION ALL SELECT 10 UNION ALL SELECT 20) ORDER BY x;");
}

#[test]
fn window_functions_002_C001() {
    compare_script("window-functions", "window-functions-002", "C001", "SELECT x, sum(x) OVER (ORDER BY x ROWS BETWEEN 1 PRECEDING AND CURRENT ROW) FROM (SELECT 10 AS x UNION ALL SELECT 20 UNION ALL SELECT 30) ORDER BY x;");
}

#[test]
fn misc_uuid_001_C001() {
    compare_script("misc-uuid", "misc-uuid-001", "C001", "SELECT length(uuid()), substr(uuid(),15,1), uuid_str(uuid_blob(uuid())) IS NOT NULL;");
}

#[test]
fn misc_regexp_001_C001() {
    compare_script("misc-regexp", "misc-regexp-001", "C001", "SELECT 'abc' REGEXP 'a.c', 'abc' REGEXP '^b';");
}

#[test]
fn misc_series_001_C001() {
    compare_script("misc-series", "misc-series-001", "C001", "SELECT count(*), sum(value) FROM generate_series(1,5);");
}

#[test]
fn misc_decimal_001_C001() {
    compare_script("misc-decimal", "misc-decimal-001", "C001", "SELECT decimal_add('1.10','2.25'), decimal_cmp('2','10');");
}

#[test]
fn misc_basexx_001_C001() {
    compare_script("misc-basexx", "misc-basexx-001", "C001", "SELECT base64(X'01FF'), hex(base64('Af8='));");
}

#[test]
fn misc_rot13_001_C001() {
    compare_script("misc-rot13", "misc-rot13-001", "C001", "SELECT rot13('Hello'), rot13(rot13('Hello'));");
}

#[test]
fn misc_totype_001_C001() {
    compare_script("misc-totype", "misc-totype-001", "C001", "SELECT tointeger('42'), tointeger('1.5') IS NULL, toreal('2.5');");
}

#[test]
fn misc_uint_001_C001() {
    compare_script("misc-uint", "misc-uint-001", "C001", "SELECT 'x9' < 'x10' COLLATE uint, 'x9' < 'x10';");
}

#[test]
fn misc_ieee754_001_C001() {
    compare_script("misc-ieee754", "misc-ieee754-001", "C001", "SELECT ieee754(2.5), ieee754_mantissa(2.5), ieee754_exponent(2.5);");
}

#[test]
fn misc_percentile_001_C001() {
    compare_script("misc-percentile", "misc-percentile-001", "C001", "SELECT median(x), percentile(x,25) FROM (SELECT 1 AS x UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4);");
}

#[test]
fn misc_prefixes_001_C001() {
    compare_script("misc-prefixes", "misc-prefixes-001", "C001", "SELECT count(*) FROM prefixes('abc');");
}

#[test]
fn misc_sha1_001_C001() {
    compare_script("misc-sha1", "misc-sha1-001", "C001", "SELECT sha1('abc');");
}

#[test]
fn misc_shathree_001_C001() {
    compare_script("misc-shathree", "misc-shathree-001", "C001", "SELECT lower(hex(sha3('abc',256)));");
}

#[test]
fn misc_urifuncs_001_C001() {
    compare_script("misc-urifuncs", "misc-urifuncs-001", "C001", "SELECT sqlite3_uri_parameter('main','vfs') IS NULL, sqlite3_uri_boolean('main','ro',0);");
}

#[test]
fn misc_zorder_001_C001() {
    compare_script("misc-zorder", "misc-zorder-001", "C001", "SELECT zorder(3,5), unzorder(zorder(3,5),2,0), unzorder(zorder(3,5),2,1);");
}

#[test]
fn pragma_surface_001_C002() {
    compare_script("pragma-surface", "pragma-surface-001", "C002", "PRAGMA application_id; PRAGMA application_id=42; PRAGMA application_id;");
}

#[test]
fn pragma_surface_001_C003() {
    compare_script("pragma-surface", "pragma-surface-001", "C003", "PRAGMA schema_version; CREATE TABLE t(a); PRAGMA schema_version;");
}

#[test]
fn pragma_surface_001_C004() {
    compare_script("pragma-surface", "pragma-surface-001", "C004", "PRAGMA cache_size; PRAGMA cache_size=100; PRAGMA cache_size;");
}

#[test]
fn pragma_surface_001_C005() {
    compare_script("pragma-surface", "pragma-surface-001", "C005", "PRAGMA recursive_triggers; PRAGMA recursive_triggers=ON; PRAGMA recursive_triggers;");
}

#[test]
fn pragma_surface_001_C006() {
    compare_script("pragma-surface", "pragma-surface-001", "C006", "PRAGMA defer_foreign_keys; PRAGMA defer_foreign_keys=1; PRAGMA defer_foreign_keys;");
}

#[test]
fn pragma_surface_001_C007() {
    compare_script("pragma-surface", "pragma-surface-001", "C007", "PRAGMA query_only; PRAGMA query_only=1; PRAGMA query_only;");
}

#[test]
fn pragma_surface_001_C008() {
    compare_script("pragma-surface", "pragma-surface-001", "C008", "PRAGMA temp_store; PRAGMA temp_store=2; PRAGMA temp_store;");
}

#[test]
fn pragma_surface_001_C009() {
    compare_script("pragma-surface", "pragma-surface-001", "C009", "PRAGMA automatic_index; PRAGMA automatic_index=0; PRAGMA automatic_index;");
}

#[test]
fn pragma_surface_001_C010() {
    compare_script("pragma-surface", "pragma-surface-001", "C010", "PRAGMA ignore_check_constraints; PRAGMA ignore_check_constraints=1; PRAGMA ignore_check_constraints;");
}

#[test]
fn pragma_surface_001_C011() {
    compare_script("pragma-surface", "pragma-surface-001", "C011", "PRAGMA case_sensitive_like=ON; SELECT 'abc' LIKE 'A%'; PRAGMA case_sensitive_like=OFF; SELECT 'abc' LIKE 'A%';");
}

#[test]
fn pragma_surface_001_C012() {
    compare_script("pragma-surface", "pragma-surface-001", "C012", "CREATE TABLE ic(a CHECK(a>0)); PRAGMA integrity_check; PRAGMA quick_check;");
}

#[test]
fn pragma_surface_002_C002() {
    compare_script("pragma-surface", "pragma-surface-002", "C002", "CREATE TABLE pt(a INTEGER PRIMARY KEY, b TEXT REFERENCES pt(a)); CREATE INDEX pi ON pt(b); SELECT count(*) FROM pragma_table_info('pt'); SELECT count(*) FROM pragma_foreign_key_list('pt'); SELECT count(*) FROM pragma_index_list('pt');");
}
