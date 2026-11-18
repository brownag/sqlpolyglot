# Project Goals (`planning/GOALS.md`)

Last Updated: 2026-11-17
Source State: none (full workspace audit)

## Circled Goal (Active)

- **[G1]** Every registry dialect name and alias is covered by a transpile test that asserts exact output.
  - Why now: unblocks dialect-list decisions (G2) and any crate upgrade (G6).
  - Exit criteria: testthat iterates `sql_supported_dialects()` over a fixed statement; ~10 golden pairs assert exact output; `mssql` and `sqlserver` produce identical output; `test_dir` passes.
  - First action: run `sql_transpile("SELECT 1", a, b)` for all 19×19 pairs and record any errors or empty results.

## Sequenced Goals (Dependency DAG)

- **[G2]** Decide and enforce the `db2`/`firebird`/`hsql` status (support or document removal).
  - Depends on: G1

## Independent Backlog

- **[G3]** Verify an offline build from a clean checkout using `vendor.tar.xz`.
- **[G4]** Add a drift check for roxygen, `NAMESPACE`, and extendr wrappers.
- **[G5]** Replace `DESCRIPTION` placeholders (needs owner name and email).
- **[G8]** Reconcile `PR_SUMMARY.md` claims with the code, then commit or drop it.

## Deferred

- **[G6]** Upgrade `polyglot-sql` past 0.5.16.
  - Trigger: a newer `polyglot-sql` release, or a need for `db2`/`firebird`.

## Blocked / Pruned

- **[G7]** Verify the Windows build.
  - Status: Blocked / External Watch
  - Reason: no Windows machine or CI runner available here.
