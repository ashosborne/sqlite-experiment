# engine-join-001 — INNER / comma joins

Confidence: observed-in-code. JOIN ... ON and FROM a,b WHERE equi-join both lower to
nested WHERE loops (src/where.c sqlite3WhereBegin); ON terms become WHERE terms for
INNER joins (src/whereexpr.c). Seven pinned shapes: ON equi-join both-sides projection,
INNER JOIN count, comma join, AS aliases, no-match empty, non-equi ON (q.a > p.a),
fresh-literal join (910033).
