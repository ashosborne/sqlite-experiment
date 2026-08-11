# ADR 0003 — Engine v5: the table scripts move off the cheat-sheet

Status: accepted (BOUND with pack v5, Ash Osborne, 2026-08-11 Europe/London, delegated).

v4 built a toy store and banned the recognizer for seven kitchen cases. v5 re-homes the remaining
eleven user-table scripts onto that store — UNIQUE (column + index) with OR IGNORE / OR REPLACE,
PK upserts (DO NOTHING / DO UPDATE SET c=excluded.c), ALTER RENAME / ADD COLUMN DEFAULT, foreign
keys (insert check → 19, ON DELETE CASCADE, DROP-parent check → 19), AFTER INSERT triggers with
new.col*N bodies, qualified names and rowid. All eleven made it; zero fell back. Their SQL is gone
from script_table (70 entries remain, all non-table expression/API pins), and a referenced-table
gate keeps the store from swallowing scripts about tables it never made (pragma projections, vtabs).

Goldens: byte-identical throughout — the implementation changed, the truth didn't. Two anti-cheat
tests insert runtime values (one plain, one through the FK rules) that no lookup table can contain.

Honest limit: UNIQUE/FK/triggers here are in-memory rules over vectors — not SQLite's btree, not a
planner, not durable. The still_recognizer list now carries a per-ID reason. SQLite is not migrated.
