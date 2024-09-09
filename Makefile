# Define variables
CARGO = cargo
TARGET = target/debug/metasurf-public-api  
BUILD_DIR = target

# Default target
all: build

# Build the Rust project
build:
	$(CARGO) build

# Run tests for the Rust project
test:
	$(CARGO) test

# Run the Rust project
run: build
	$(CARGO) run

# Check the Rust project (lints, warnings, etc.)
check:
	$(CARGO) check

# Clean up build artifacts
clean:
	$(CARGO) clean

# Format Rust code
format:
	$(CARGO) fmt

# Fix coding style and lints
cs-fix:
	$(CARGO) fmt
	$(CARGO) clippy --fix --allow-dirty --allow-staged

# Build the documentation
doc:
	$(CARGO) doc

# Run benchmarks (requires nightly Rust)
bench:
	$(CARGO) bench

# Build a release version
release:
	$(CARGO) build --release

# Clean up build artifacts and target directory
clean-all: clean
	rm -rf $(BUILD_DIR)

.PHONY: all build test run check clean format doc bench release clean-all cs-fix
