# CANDIDATES — ddl-schema (Phase A, unbound)

All `candidate`, confidence `observed-in-code`. STOP for human bind.

| # | Candidate | Evidence | Why in slice |
| --- | --- | --- | --- |
| 001 | Table/view lifecycle | `src/build.c:1225` (StartTable), `:2670` (EndTable), `:3023` (CreateView), `:3528` (DropTable) | Core schema-object DDL incl. sqlite_master writes |
| 002 | Index lifecycle | `src/build.c:3974` (CreateIndex), `:4629` (DropIndex) | Index DDL incl. UNIQUE + partial/expression indexes |
| 003 | ALTER TABLE family | `src/alter.c:124` (rename table), `:313` (add column), `:599` (rename column), `:2250` (drop column) | Schema rewriting with reference fixup |
