# Deferred — testgen/error-status-api (run 4)

Operator-directed DEFERs (run-4 paste):
- Pinning the exact English wording of error messages as a contract (RECORD captures verbatim text;
  wording-drift classification stays a human review call).
- UTF-16 twins (`sqlite3_errmsg16`, errcode over UTF-16 paths).
- `sqlite3_error_offset` corpus (offset-per-syntax-error matrix).
- `sqlite3_limit` and `sqlite3_status64`/`db_status` — belong to error-status-api-002/-003, not in this batch.
