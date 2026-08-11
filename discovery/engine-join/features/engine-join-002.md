# engine-join-002 — three-table / self-join / aggregates / GROUP BY / LEFT JOIN

Confidence: observed-in-code. Multi-way joins chain loops; LEFT JOIN emits NULL rows for
unmatched left rows (src/where.c LEFT JOIN handling); GROUP BY groups via sorter
(src/select.c). Seven pinned shapes incl. A JOIN B JOIN C, aliased self-join,
count/min/max over a join, GROUP BY over join + single table, LEFT JOIN row shape and
count(*) vs count(right.col).
