#' Transpile SQL from one dialect to another
#'
#' A vectorized wrapper around the Rust-based `transpile_sql` function.
#'
#' @param sql A character vector of SQL statements to transpile.
#' @param from The source SQL dialect (e.g., "mysql", "postgres").
#' @param to The target SQL dialect (e.g., "bigquery", "snowflake").
#'
#' @return A character vector of transpiled SQL statements.
#'
#' @export
#'
#' @examples
#' \dontrun{
#' sql_transpile("SELECT * FROM table WHERE id = 1", from = "mysql", to = "postgres")
#' }
sql_transpile <- function(sql, from, to) {
  if (length(sql) == 0) {
    return(character(0))
  }

  # Use vapply to ensure we return a character vector
  vapply(sql, function(s) {
    res <- transpile_sql(s, from, to)
    if (length(res) == 0) {
      return(NA_character_)
    }
    return(res[1])
  }, character(1))
}

#' Get all supported SQL dialects
#'
#' Returns a character vector of all supported dialects for the `polyglot` engine.
#'
#' @return A character vector of dialect names.
#'
#' @export
sql_supported_dialects <- function() {
  # In a production implementation, this could be dynamically fetched from the Rust backend.
  # For now, we provide a comprehensive list of commonly supported dialects.
  c(
    "postgres", "postgresql", "mysql", "sqlite", "sqlite3", 
    "bigquery", "snowflake", "redshift", "duckdb", "oracle", 
    "sqlserver", "mssql", "teradata", "clickhouse", "firebird",
    "db2", "mariadb", "hsql", "presto", "trino", "spark", "hive"
  )
}
