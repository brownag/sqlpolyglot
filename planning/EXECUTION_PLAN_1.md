# Execution Plan 1 -- G1: Registry dialect coverage tests

Source: planning/GOALS.md, Circled Goal G1.

## Goal & Scope

- **Goal**: Every dialect name and alias in the Rust `DIALECT_REGISTRY` is covered by a transpile test that asserts exact output for golden pairs and no error for all pairs.
- **In scope**: `tests/testthat/test-transpile.R` only. Test code, no production code changes.
- **Out of scope**: changing the registry or any engine behavior; db2/firebird/hsql (G2); crate upgrades (G6); CI, DESCRIPTION, Windows (G3, G4, G5, G7).
- **Verbatim constraints**: "Every registry dialect name and alias is covered by a transpile test that asserts exact output." Exit criteria: testthat iterates `sql_supported_dialects()` over a fixed statement; ~10 golden pairs assert exact output; `mssql` and `sqlserver` produce identical output; `test_dir` passes.

## Decision log

- 2026-11-17: Planning artifacts are tracked; commit them separately as `docs(planning):`.
- 2026-11-17: Golden-pair expected outputs are hand-written from dialect rules, not snapshotted from the engine. A mismatch between engine output and a hand-written expectation is a STOP-and-report, never a silent edit of the expectation.
- 2026-11-17: Coverage is every pair of the 19 names (19x19) returns a non-empty, non-NA string with no error.
- 2026-11-17: Acceptance is testthat plus `cargo check`.
- 2026-11-17 (defaulted, to confirm): single wave, single task, since all test edits share one file. Worker model: small (haiku-class). Escalation: mid-tier (sonnet-class).

## Baseline (pre-wave-1)

- Install: `R CMD INSTALL --no-test-load .` from repo root.
- R tests: `Rscript -e 'testthat::test_dir("tests/testthat", reporter="summary")'` -> 8 expectations pass (observed 2026-11-17).
- Rust: `cargo check --manifest-path src/rust/Cargo.toml` -> passes (observed 2026-11-17).

## Hazards (from recon)

- `sql_transpile` vectorizes via `vapply` and returns `NA` when the engine returns nothing. Check for NA explicitly.
- The package must be installed before testthat runs, or `sql_transpile` is not found.
- `tests/testthat/test-transpile.R` has a comment saying "scaffolding"; replace that test style, don't copy the comment.

## Wave table

| Wave | Task | Files | Depends on |
|---|---|---|---|
| 1 | 1.1.1 | tests/testthat/test-transpile.R | -- |

## Task specs

### Task 1.1.1 -- Registry coverage tests

- **Repo**: sqlpolyglot (`/home/andrew/workspace/sqlpolyglot`)
- **Files in scope**:
  - Must edit: `tests/testthat/test-transpile.R`
  - Can create: none
  - Scratch: none needed
- **Depends on**: none
- **Spec**:
  1. Add a test that calls `sql_supported_dialects()` and, for every (from, to) pair of the returned names, runs `sql_transpile("SELECT 1", from, to)`. Assert no error and a single non-NA, non-empty string. Use `expect_no_error` and `expect_false(is.na(...))`.
  2. Add a test asserting alias equivalence: for each pair (`mssql`, `sqlserver`), (`postgres`, `postgresql`), (`mysql`, `mariadb`), (`sqlite`, `sqlite3`), the output of the same statement is identical.
  3. Add golden-pair tests with exact expected strings. Derive expectations from dialect rules, then verify against engine output. Pairs (each a `test_that` block or a row in one table-driven test):
     - mysql -> postgres: `SELECT IFNULL(a, 'x')` -> `COALESCE(a, 'x')`
     - postgres -> mysql: `SELECT COALESCE(a, b)` -> `COALESCE(a, b)`
     - sqlite -> postgres: `SELECT a FROM t LIMIT 5` -> unchanged
     - mssql -> postgres: `SELECT TOP 5 a FROM t` -> `SELECT a FROM t LIMIT 5`
     - postgres -> mssql: `SELECT a FROM t LIMIT 5` -> `SELECT TOP 5 a FROM t`
     - snowflake -> postgres: `SELECT IFF(a > 0, 1, 0)` -> `SELECT CASE WHEN a > 0 THEN 1 ELSE 0 END`
     - bigquery -> postgres: `SELECT IFNULL(a, 0)` -> `SELECT COALESCE(a, 0)`
     - duckdb -> postgres: `SELECT a FROM t` -> `SELECT a FROM t`
     - mysql -> sqlite: `SELECT IFNULL(a, 'x') FROM t` -> `SELECT IFNULL(a, 'x') FROM t`
     - hive -> spark: `SELECT a FROM t` -> `SELECT a FROM t`
     Each expectation must match engine output exactly. If the engine output differs from a hand-written expectation, STOP and report the pair, the expected string, and the actual string. Do not edit the expectation.
  4. Keep the existing four tests. Do not remove them.
- **Anchors**:
  - `tests/testthat/test-transpile.R` lines 1, 11, 16, 23 (existing `test_that` blocks)
  - `R/transpile.R` `sql_transpile` (line 17), `sql_supported_dialects` (line 40)
  - `src/rust/src/lib.rs` `DIALECT_REGISTRY` (line 7), `parse_dialect` (line 30), `get_dialects` (line 40)
- **Reference implementation**: the existing `test_that` blocks in `tests/testthat/test-transpile.R`.
- **Constraints (do not touch)**: any file other than `tests/testthat/test-transpile.R`; `R/`; `src/`; `DESCRIPTION`; `NAMESPACE`; `man/`; `planning/` except the worker's own report in STATE_1.md.
- **Acceptance**:
  - `R CMD INSTALL --no-test-load .` succeeds
  - `Rscript -e 'testthat::test_dir("tests/testthat", package="sqlpolyglot", load_package="installed", reporter="summary", stop_on_failure=TRUE)'` exits 0 with zero failures
  - `cargo check --manifest-path src/rust/Cargo.toml` passes (unchanged, must stay green)
