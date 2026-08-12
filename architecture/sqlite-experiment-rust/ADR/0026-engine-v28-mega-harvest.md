# ADR 0026 — engine v28: mega harvest

Status: accepted (pack v28 BOUND, Ash Osborne, delegated autonomy run 38)

## Context

A wide harvest across deferred none C-API surfaces, bundled misc extensions, and thin
Partial gaps. Depth over speed; under-claim.

## The two-level-pin catch (important)

The pinned baseline has TWO faces: the CLI (with shell extensions) and the bare
amalgamation used by harnesses. My run-38 harness force-linked several ext/misc `.c`
files, which masked whether a function is actually in the baseline. Cross-checking the
prior run-11 misc-* goldens decided each case honestly:

- `misc-compress-001` / `misc-nextchar-001` / `misc-wholenumber-001` /
  `misc-completion-001` prior goldens are **rc=0** — bundled in the pinned amalgamation,
  so implementing them is honest and matches the baseline.
- `misc-percentile-001` prior golden is **rc=1** — percentile/median are NOT in the
  pinned build. I initially implemented them (and froze 6 goldens), then **reverted and
  dropped that batch** rather than break the prior bare-build golden. Providing a
  function the baseline lacks would be greenwash.

## Attempted -> outcome

| Card | Outcome |
| --- | --- |
| exec-convenience-api-002 (get_table) | none -> **full** (header/rows/NULL/0x0/error/non-query pinned) |
| error-status-api-003 (status64/db_status) | none -> **partial** (MEMORY_USED + SCHEMA_USED real; other counters honest-zero) |
| auth-callback-api-002 (column IGNORE) | none -> **full** (IGNORE->NULL in results, WHERE, aggregates, *) |
| misc-compress-001 | none -> **partial** (round-trips real; zlib byte-format residual) |
| misc-nextchar-001 | none -> **full** |
| misc-wholenumber-001 | none -> **partial** (bounded generator; vtab-core residual) |
| misc-completion-001 | none -> **partial** (keyword/schema candidates; full shell completion residual) |
| parser-grammar-002 | none -> **partial** (unquoted keywords-as-identifiers real; quoted-reserved-word-as-table-name residual) |
| tokenizer-002 | tightened (string/comment-aware complete now real) |
| pragma-surface-002 | tightened (function_list / pragma_list TVFs) |
| percentile/median | **not implemented** (absent from pinned build) |

Plus composed full cards engine-harvest28-001/002/003/004/006/007/008 for the frozen
batches (002 status is partial-shaped but its pins are fully real, so its composed card
is full for exactly those pins).

## Engine holes opened (real, reused everywhere)

`BETWEEN` in the expression parser; blob-vs-text inequality in `=`; `SELECT *` expansion
over a single store table; quoted-identifier parsing in `ident()`; CREATE VIRTUAL TABLE
registration + bounded wholenumber generator; completion/pragma-registry TVFs.

## Deliberate residuals

zlib byte-format for compress; vtab-core general module system; full shell completion;
quoted reserved-word table names; the rest of the status/db_status op matrix; STAT4;
planner load. WAL/VACUUM/blob/conn untouched.

## Consequences

33 goldens replay byte-identical; anti-cheat proves runtime compress/next_char; cargo
644/644; prior goldens (incl. the bare-build misc-percentile-001 rc=1) intact.
