use extendr_api::prelude::*;
use polyglot_sql::{DialectType, transpile};

/// The single source of truth for supported dialects and their string aliases.
/// This registry handles both parsing (via `parse_dialect`) and 
/// enumeration (via `get_dialects`).
const DIALECT_REGISTRY: &[(&str, DialectType)] = &[
    ("postgres", DialectType::PostgreSQL),
    ("postgresql", DialectType::PostgreSQL),
    ("mysql", DialectType::MySQL),
    ("mariadb", DialectType::MySQL),
    ("sqlite", DialectType::SQLite),
    ("sqlite3", DialectType::SQLite),
    ("bigquery", DialectType::BigQuery),
    ("snowflake", DialectType::Snowflake),
    ("redshift", DialectType::Redshift),
    ("duckdb", DialectType::DuckDB),
    ("oracle", DialectType::Oracle),
    ("sqlserver", DialectType::TSQL),
    ("mssql", DialectType::TSQL),
    ("teradata", DialectType::Teradata),
    ("clickhouse", DialectType::ClickHouse),
    ("hive", DialectType::Hive),
    ("spark", DialectType::Spark),
    ("trino", DialectType::Trino),
    ("presto", DialectType::Presto),
];

/// Parses a dialect string using the registry.
fn parse_dialect(dialect: &str) -> Result<DialectType, String> {
    let d = dialect.to_lowercase();
    DIALECT_REGISTRY.iter()
        .find(|(name, _)| *name == d)
        .map(|(_, variant)| *variant)
        .ok_or_else(|| format!("Unknown dialect: '{}'", dialect))
}

/// Returns all unique supported dialect names registered in the engine.
#[extendr]
fn get_dialects() -> Vec<String> {
    let mut names: Vec<String> = DIALECT_REGISTRY.iter().map(|(n, _)| n.to_string()).collect();
    names.sort();
    names.dedup();
    names
}

/// Transpiles SQL from one dialect to another.
///
/// @param sql The SQL string to transpile.
/// @param read_dialect The source SQL dialect.
/// @param write_dialect The target SQL dialect.
///
/// @return A vector of transpiled SQL strings.
/// @export
#[extendr]
fn transpile_sql(sql: &str, read_dialect: &str, write_dialect: &str) -> Result<Vec<String>, Error> {
    let from = parse_dialect(read_dialect).map_err(|e| Error::Other(e))?;
    let to = parse_dialect(write_dialect).map_err(|e| Error::Other(e))?;

    transpile(sql, from, to).map_err(|e| Error::Other(format!("Transpilation error: {}", e)))
}

extendr_module! {
    mod sqlpolyglot;
    fn transpile_sql;
    fn get_dialects;
}
