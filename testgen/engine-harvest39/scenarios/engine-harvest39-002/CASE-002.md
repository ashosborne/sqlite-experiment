# engine-harvest39-002-C002 — run-11 oneshot characterization

Feature: `engine-harvest39-002` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: create_module names appear; a touched pragma TVF joins the lazy list (see harness).

## Observables

- `after_create_module` = `mymod|pragma_module_list`
- `touch_tvf` = `3`
- `after_tvf` = `mymod|pragma_collation_list|pragma_module_list`
