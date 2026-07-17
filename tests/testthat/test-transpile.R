test_that("sql_transpile performs simple transpilation", {
  # Test case: MySQL IFNULL to Postgres COALESCE
  sql_mysql <- "SELECT IFNULL(column_name, 'default') FROM table_name"
  
  # We expect it to work. If the crate supports it, it should return COALESCE.
  # Given we are scaffolding, we implement the test as requested.
  result <- sql_transpile(sql_mysql, from = "mysql", to = "postgres")
  expect_match(result[1], "COALESCE", ignore.case = TRUE)
})

test_that("sql_transpile handles errors for unknown dialects", {
  expect_error(sql_transpile("SELECT 1", from = "nonexistent", to = "postgres"), "Unknown dialect")
  expect_error(sql_transpile("SELECT 1", from = "mysql", to = "nonexistent"), "Unknown dialect")
})

test_that("sql_supported_dialects returns a character vector", {
  dialects <- sql_supported_dialects()
  expect_type(dialects, "character")
  expect_true(length(dialects) > 0)
  expect_true("mysql" %in% dialects)
})

test_that("sql_transpile handles multiple inputs", {
  sqls <- c("SELECT 1", "SELECT 2")
  results <- sql_transpile(sqls, from = "mysql", to = "postgres")
  expect_length(results, 2)
  expect_type(results, "character")
})
