# STATE 1 -- Orchestration 1 (G1: registry dialect coverage tests)

Plan: planning/EXECUTION_PLAN_1.md
Source: planning/GOALS.md (Circled Goal G1)

## Baseline checklist (pre-wave-1)

- [x] Repo: `/home/andrew/workspace/sqlpolyglot`, branch `main`, single worktree, clean code tree (PR_SUMMARY.md and planning/ untracked)
- [x] `R CMD INSTALL --no-test-load .` (confirmed 2026-11-17)
- [x] testthat suite: 8 expectations pass (confirmed 2026-11-17)
- [x] `cargo check --manifest-path src/rust/Cargo.toml` passes (confirmed 2026-11-17)

## Wave board

| Wave | Task | Status |
|---|---|---|
| 1 | 1.1.1 Registry coverage tests | done (code commit 868385b) |

## Task reports

_No reports yet._

## 1.1.1

Status: BLOCKED before any edit.

- Baseline `R CMD INSTALL --no-test-load .` succeeded; `cargo check` passed.
- Acceptance command `Rscript -e 'testthat::test_dir("tests/testthat", reporter="summary")'` fails on the unmodified repo: 4 errors, "could not find function sql_transpile" / "sql_supported_dialects". The package is not attached and there is no tests/testthat.R or helper to attach it.
- With `library(sqlpolyglot)` first, the existing 4 tests pass (exit 0).
- No test edits made. No commits.

Question for coordinator: may the acceptance command be run with `library(sqlpolyglot)` prepended, or should a tests/testthat.R or helper be added (outside current scope)?

### 1.1.1 update (supersedes BLOCKED status above)

Status: STOPPED on golden-pair mismatch. Nothing committed.

Deviations:
- Acceptance command changed per coordinator to `testthat::test_dir("tests/testthat", package="sqlpolyglot", load_package="installed", reporter="summary", stop_on_failure=TRUE)`.
- `sql_transpile` returns named vectors (vapply USE.NAMES), so golden comparisons wrap actual output in `unname()`. Expectations unchanged.

Results with the edited test file (uncommitted):
- All-pairs test (361 pairs): pass.
- Alias equivalence test (4 pairs, `SELECT a FROM t` -> postgres): pass.
- Original 4 tests: pass.
- Golden pairs: 8 of 10 pass. 2 mismatches, expectations NOT edited:
  1. postgres -> mssql, input `SELECT a FROM t LIMIT 5`. Expected `SELECT TOP 5 a FROM t`. Actual `SELECT a FROM t LIMIT 5`.
  2. mysql -> sqlite, input `SELECT IFNULL(a, 'x') FROM t`. Expected `SELECT IFNULL(a, 'x') FROM t`. Actual `SELECT COALESCE(a, 'x') FROM t`.

Question: keep the spec expectations and treat these as engine bugs to report, or change the expectations? Awaiting decision. No commit until resolved.

#### 1.1.1 final

Decisions:
1. postgres -> mssql (`SELECT a FROM t LIMIT 5` -> `SELECT TOP 5 a FROM t`): expectation kept. Pair excluded from the table loop and tested in its own block, skipped with `engine gap: LIMIT is not rewritten to TOP for mssql`.
2. mysql -> sqlite (`SELECT IFNULL(a, 'x') FROM t`): expected string changed to `SELECT COALESCE(a, 'x') FROM t` (engine output, valid SQLite).

Verification:
- Acceptance command (package="sqlpolyglot", load_package="installed"): exit 0, 1 skip, 0 failures.
- `cargo check`: passes.
- Diff on test-transpile.R: 65 additions, 0 deletions.
