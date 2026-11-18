# PR: Implement unified dialect registry in Rust backend

## Description
Refactored the Rust-to-R bridge to use a single `DIALECT_REGISTRY` constant. This replaces the previous approach of maintaining a separate manual list in R and a `match` statement in Rust, consolidating all dialect mappings into one source of truth.

## Key Changes
* **Unified Source of Truth**: All supported dialects and their string aliases are now defined in a single `DIALECT_REGISTRY` array in `src/rust/src/lib.rs`.
* **Improved Robustness**: The `parse_dialect` function now performs lookups against this registry, ensuring that aliases (e.g., `mssql` and `sqlserver`) are correctly mapped to the same `DialectType`.
* **Dynamic Discovery**: The `get_dialects` function now iterates over the registry to return all unique supported names, allowing the R function `sql_supported_dialects()` to stay perfectly in sync with the Rust engine without manual updates.
* **Performance**: Maintains $O(N)$ lookup performance, which is negligible for the small number of supported dialects.

## Impact
* Eliminates the risk of the R documentation/discovery being out of sync with the actual Rust engine capabilities.
* Simplifies the addition of new dialects to a single line of code.
* Reduces maintenance overhead by removing duplicate logic between the Rust and R layers.
