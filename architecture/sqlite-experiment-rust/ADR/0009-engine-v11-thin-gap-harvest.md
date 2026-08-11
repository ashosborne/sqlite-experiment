# ADR 0009 — engine v11: thin-gap harvest (pack v11)

Date: 2026-08-12 · Status: BOUND (supersedes pack v10; v1–v10 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 21)

## Context
Scoreboard after run 20: 26 full / 72 partial / 103 none. COVERAGE's partial notes named
bounded missing pieces; closing them (plus HAVING/DISTINCT depth) grows honest fulls
faster than WAL or a planner would.

## Decision
Implement every named thin gap for real (computed, never pinned):
aggregates gained DISTINCT, FILTER (WHERE …) and HAVING (evaluated over groups,
aggregates allowed); ORDER BY terms gained COLLATE (uint/rot13/nocase/decimal),
ASC/DESC and NULLS FIRST/LAST with C's default NULL placement; sha1_query/sha3_query
hash real query results using the extension's exact stream protocol
(S{n}:sql · R · N/I/F/T/B tags, big-endian value images); base85 encode/decode with
SQLite's numeral set and trailing newline; ieee754_from_blob/to_blob; decimal(X)
canonicalization, decimal_pow2 in the extension's +D.DDDe±EE form, decimal collation;
tointeger real/blob/overflow strictness; uuid_str/uuid_blob canonicalization;
printf %w and the # alternate flag; FK actions completed (ON UPDATE CASCADE/SET NULL,
ON DELETE SET DEFAULT); trigger surface completed (INSTEAD OF on views, DROP TRIGGER,
RAISE(ABORT) → rc 19, UPDATE OF column filters, recursive_triggers recursion with a
depth cap); ALTER RENAME COLUMN / DROP COLUMN; upsert DO UPDATE … WHERE.
33 cases frozen on pinned C (two-run gate, delegated stamp) across 7 harvest slices;
all replay byte-identical; zero deferrals. Two executor bugs found by goldens and
fixed: `_` treated as a word boundary (FROM matched inside ieee754_from_blob), and the
kitchen SELECT silently dropping ORDER BY clauses it could not parse.

## Consequences
cargo 243/243; anti-cheat 12/12 (runtime HAVING/DISTINCT, runtime sha1_query + base85
round-trip); prior goldens byte-identical. Scoreboard: full 26 → 45. Still NOT:
planner/flattening, WAL, RAISE(IGNORE/FAIL/ROLLBACK), INSTEAD OF UPDATE/DELETE,
thousands-separator printf flag, decimal_exp, window family breadth. NOT migrated.
