# engine-harvest40-006-C003 — run-11 oneshot characterization

Feature: `engine-harvest40-006` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: default ALTER RENAME rewrites referencing view SQL with the quoted new name; LEGACY_ALTER leaves the view text untouched (see harness).

## Observables

- `alter_default` = `rc=0 err=-`
- `view_sql_default` = `CREATE VIEW v AS SELECT a FROM "u"`
- `legacy_state` = `1`
- `alter_legacy` = `rc=0 err=-`
- `view_sql_legacy` = `CREATE VIEW v AS SELECT a FROM t`
