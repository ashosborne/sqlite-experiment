# ADR 0014 — engine v16: CHECK on UPDATE (pack v16)

Date: 2026-08-12 · Status: BOUND (supersedes pack v15; v1–v15 at versions/)
Binder: Ash Osborne (FULL_AUTONOMY charter, run 26 — focused single-gap loop)

## Context
dml-codegen-002 stayed partial after v15 for exactly one reason: CHECK constraints
were only evaluated on INSERT. UPDATE could silently violate them.

## Decision
CHECKs (column-level, and table-level `CHECK(a < b)` constraints — which this run
started capturing at all; they are now also enforced on INSERT) are evaluated against
the POST-update row image during statement planning. Conflict modes are wired to the
v15 snapshot transactions exactly as pinned on C: plain/OR ABORT undoes the whole
statement (nothing applied — pinned: rows (1,9) unchanged); OR FAIL keeps earlier row
changes of the same statement (pinned: (6,9)); OR IGNORE skips the violating row and
continues; OR ROLLBACK unwinds the whole transaction (autocommit→1, COMMIT then
errors). NULL CHECK results pass (SQL three-valued semantics, pinned). NOT NULL is
validated on the post-update image as well (unpinned but C-correct direction).
File twins pin durability: a passing UPDATE survives reopen; a failing one is absent.

## Consequences
15 cases frozen (8 script + 7 bespoke; two-run gate, delegated stamp), all replaying
byte-identical; cargo 377/377; anti-cheat 24/24. dml-codegen-002 honestly flips to
full: conflict matrix IGNORE/REPLACE/ABORT/FAIL/ROLLBACK + CHECK/NOT NULL/UNIQUE/FK
enforced on INSERT and UPDATE. Still out (noted on other cards): expression-DEFAULT,
generated columns, OR REPLACE×CHECK interplay (unpinned — C treats CHECK failures as
non-replaceable errors; not claimed). NOT migrated.
