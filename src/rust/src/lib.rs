use extendr_api::prelude::*;
use polyglot_sql::{DialectType, transpile};

/// Helper function to parse dialect strings into DialectType enums.
/// Handles case-insensitivity and common name variations.
fn parse_dialect(dialect: &str) -> Result<DialectType, String> {
    let d = dialect.to_lowercase();
    match d.as_str() {
        "postgres" | "postgresql" => Ok(DialectType::PostgreSQL),
        "mysql" => Ok(DialectType::MySQL),
        "sqlite" | "sqlite3" => Ok(DialectType::SQLite),
        "bigquery" => Ok(DialectType::BigQuery),
        "snowflake" => Ok(DialectType::Snowflake),
        "redshift" => Ok(DialectType::Redshift),
        _ => Err(format!("Unknown dialect: '{}'. Please check supported dialects.", dialect)),
    }
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

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod sqlpolyglot;
    fn transpile_sql;
}
