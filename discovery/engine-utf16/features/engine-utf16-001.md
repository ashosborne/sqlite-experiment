# engine-utf16-001 — UTF-16 prepare family

Confidence: observed-in-code (composed slice, run 29).
sqlite3_prepare16 / prepare16_v2 / prepare16_v3 over real UTF-16LE buffers: compile,
step/column like the UTF-8 twins, UTF-16 pzTail, nByte in bytes, whitespace-only OK+NULL,
syntax-error rc/errmsg, UTF-16 DDL/DML, non-ASCII literals (accents + surrogate pairs),
bind_text16 round trip.
Frozen scope = run-29 pinned cases (pack v19). Endian: native-LE; BOM not claimed.
