# engine-harvest37-005-C001 — run-11 oneshot characterization

Feature: `engine-harvest37-005` · Kind: characterization · Assert mode: RECORDED (was TO_BE_RECORDED)
Matrix gate: run-11 full-autonomy charter. Determinism gate: two independent RECORD runs byte-matched.

## Boundary invoke

Bespoke C API sequence: sqlite3_config after init: threading modes/MEMSTATUS/URI -> MISUSE; LOG + PCACHE_HDRSZ legal; bad op MISUSE (see harness).

## Observables

- `single_after` = `21`
- `multi_after` = `21`
- `serialized_after` = `21`
- `memstatus_after` = `21`
- `uri_after` = `21`
- `log_after` = `0`
- `pcache_hdrsz_rc` = `0`
- `hdr_positive` = `1`
- `badop` = `21`
