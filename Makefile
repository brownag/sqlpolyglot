SHELL := /bin/bash

PKG      ?= sqlpolyglot
VERSION  := $(shell sed -n 's/^Version: //p' DESCRIPTION)
TARBALL  := $(PKG)_$(VERSION).tar.gz

.PHONY: help install document build check test clean

.DEFAULT_GOAL := help

help: ## Show this help message
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

install: ## Install the package from the working tree
	R CMD INSTALL .

document: ## Regenerate man/ and NAMESPACE with roxygen2
	Rscript -e 'roxygen2::roxygenise()'

build: ## Build the source tarball
	R CMD build .

check: build ## Run R CMD check on the built tarball
	R CMD check --as-cran $(TARBALL)

test: ## Run testthat suite
	Rscript -e 'testthat::test_local()'

clean: ## Remove build artifacts
	rm -rf $(PKG).Rcheck $(PKG)_*.tar.gz
	rm -f src/Makevars src/Makevars.win
	rm -rf src/rust/target
