# sqlpolyglot

Transpile SQL between dialects from R. The translation engine is written in Rust (the `polyglot-sql` crate) and bound to R with [extendr](https://extendr.rs/).

## Installation

Requires R (>= 4.2), plus Rust and Cargo (`rustc` >= 1.65).

```bash
R CMD INSTALL .
```

Or, from a checkout with the `Makefile` targets:

```bash
make install
```

## Usage

```r
library(sqlpolyglot)

sql_transpile("SELECT IFNULL(a, 0) FROM t", from = "mysql", to = "postgres")
#> [1] "SELECT COALESCE(a, 0) FROM t"

sql_transpile(c("SELECT 1", "SELECT 2"), from = "postgres", to = "bigquery")
#> [1] "SELECT 1" "SELECT 2"
```

`sql_transpile()` is vectorized over `sql` and returns a character vector of the same length.

List the supported dialect names:

```r
sql_supported_dialects()
```

Unknown dialect names raise an error containing `Unknown dialect`.

## Supported dialects

| Dialect | Accepted names |
|---|---|
| PostgreSQL | `postgres`, `postgresql` |
| MySQL | `mysql`, `mariadb` |
| SQLite | `sqlite`, `sqlite3` |
| SQL Server | `sqlserver`, `mssql` |
| BigQuery | `bigquery` |
| Snowflake | `snowflake` |
| Redshift | `redshift` |
| DuckDB | `duckdb` |
| Oracle | `oracle` |
| Teradata | `teradata` |
| ClickHouse | `clickhouse` |
| Hive | `hive` |
| Spark | `spark` |
| Trino | `trino` |
| Presto | `presto` |

Names are case-insensitive. `sql_supported_dialects()` returns the same list at runtime.

## Development

| Target | Action |
|---|---|
| `make install` | Install the package from the working tree |
| `make document` | Regenerate `man/` and `NAMESPACE` with roxygen2 |
| `make test` | Run the testthat suite |
| `make check` | Build the tarball and run `R CMD check --as-cran` |
| `make clean` | Remove build artifacts |

Run `make help` for the full list.

To add a dialect, add its names to `DIALECT_REGISTRY` in `src/rust/src/lib.rs`. `sql_supported_dialects()` picks it up without R-side changes.

## License

MIT. See `LICENSE` and `LICENSE.md`.
