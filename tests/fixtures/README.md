# Test Fixtures

This directory contains reusable test data that is consumed by the project's automated tests.

A **test fixture** is a predefined piece of data or a collection of files that provides a known input or expected result for a test. Fixtures allow tests to work with consistent, reproducible data instead of having to construct all test inputs inline.

The purpose of this directory is to keep test data separate from test implementation and shared test-support code.

## Purpose

The `tests/fixtures/` directory is intended for storing files that are used as test inputs or expected outputs.

Typical examples include:

- Configuration files used to test configuration loading and parsing.
- JSON, XML, CSV, or other structured data used to test parsers and serializers.
- Sample input files used to test file-processing functionality.
- Expected output files used to compare the results of an operation.
- Valid and invalid examples used to test successful and unsuccessful execution paths.
- Binary files used to test handling of images, archives, or other binary formats.
- Larger or more complex datasets that would be impractical to define directly in Rust test code.

Fixtures should represent realistic test scenarios while remaining small and focused enough to understand and maintain.

## Relationship to `tests/common/`

The `tests/common/` directory and the `tests/fixtures/` directory have different responsibilities.

- `tests/common/` contains Rust code that supports test implementation.
- `tests/fixtures/` contains data files consumed by tests or test-support code.

For example:

| Location                                 | Purpose                                           |
| ---------------------------------------- | ------------------------------------------------- |
| `tests/common/test_user.rs`              | Defines a reusable `TestUser` structure.          |
| `tests/common/builders.rs`               | Contains functions for constructing test objects. |
| `tests/common/helpers.rs`                | Contains reusable test-support functions.         |
| `tests/fixtures/users/sample_users.json` | Contains sample user data in JSON format.         |
| `tests/fixtures/configs/valid.toml`      | Contains a valid configuration file.              |
| `tests/fixtures/configs/invalid.toml`    | Contains an invalid configuration file.           |

A Rust structure that represents test data should generally live in `tests/common/`, even when multiple tests use it.

The corresponding data files used to populate or test that structure belong in `tests/fixtures/`.

## Directory Organization

Fixtures should be organized into subdirectories according to their purpose, data format, or the functionality being tested.

For example:

```text
tests/
├── common/
│   ├── builders.rs
│   ├── helpers.rs
│   └── test_user.rs
│
├── fixtures/
│   ├── README.md
│   │
│   ├── configs/
│   │   ├── valid.toml
│   │   ├── invalid.toml
│   │   └── missing_fields.toml
│   │
│   ├── users/
│   │   ├── sample_users.json
│   │   └── empty_users.json
│   │
│   └── input/
│       ├── sample.csv
│       └── malformed.csv
│
├── authentication.rs
├── configuration.rs
└── parser.rs
```

The structure above is an example, not a requirement. Subdirectories should be introduced when they make the fixture collection easier to navigate.

Avoid creating unnecessary directory levels for only one or two files.

## Using Fixtures in Tests

Tests can access fixture files using Rust's standard library and macros.

### Compile-time inclusion

For small text fixtures, `include_str!` is often convenient:

```rust
#[test]
fn parses_valid_configuration() {
    let input = include_str!("fixtures/configs/valid.toml");

    // Use input in the test.
}
```

The `include_str!` macro includes the contents of a UTF-8 text file in the compiled binary as a string slice.

For binary fixtures, use `include_bytes!`:

```rust
#[test]
fn processes_binary_input() {
    let input = include_bytes!("fixtures/input/sample.bin");

    // Use input in the test.
}
```

The paths in these examples are relative to the Rust source file containing the macro. For example, they assume the test file is directly inside `tests/`.

### Reading fixtures at runtime

For cases where a fixture should be read from the filesystem at runtime, use `std::fs`:

```rust
use std::fs;

#[test]
fn reads_configuration_file() {
    let path = format!(
        "{}/tests/fixtures/configs/valid.toml",
        env!("CARGO_MANIFEST_DIR")
    );

    let input = fs::read_to_string(path)
        .expect("failed to read configuration fixture");

    // Use input in the test.
}
```

Using `CARGO_MANIFEST_DIR` avoids depending on the process's current working directory.

Choose compile-time inclusion or runtime file access according to the test's needs. Neither approach is universally preferable.

## Naming Conventions

Fixture filenames should describe their contents or the scenario they represent.

Prefer descriptive names:

```text
valid.toml
invalid.toml
missing_required_field.toml
empty_users.json
users_with_duplicate_ids.json
malformed.csv
```

Avoid vague names:

```text
test1.json
input2.toml
example.dat
data.txt
```

Where appropriate, use names that communicate the expected behavior or test scenario.

For example, `missing_required_field.toml` is more informative than `invalid2.toml`.

## Guidelines for Adding Fixtures

When adding a fixture, follow these guidelines:

1. **Keep fixtures deterministic.** A fixture should have stable contents so that test results do not depend on external services, random values, or the current time.

2. **Use focused examples.** Prefer small fixtures that demonstrate one specific scenario over large files containing unrelated data.

3. **Cover relevant scenarios.** When testing parsers or data validation, consider providing valid, invalid, empty, and boundary-case examples where appropriate.

4. **Make the data understandable.** Use realistic and meaningful values whenever possible. Someone reading a fixture should be able to understand its purpose without extensive investigation.

5. **Avoid unnecessary duplication.** Reuse an existing fixture when it represents the same scenario. Create a separate fixture when the new test requires meaningfully different input.

6. **Keep fixtures independent of test execution order.** Tests should not rely on another test modifying a fixture before it can be used.

7. **Do not store generated test artifacts here.** Temporary files, files created during test execution, and other generated output should normally be placed in a temporary directory instead.

8. **Avoid sensitive information.** Fixtures must not contain real credentials, API keys, personal information, or other sensitive data. Use synthetic or anonymized values.

9. **Document non-obvious fixtures.** When the purpose or expected behavior of a fixture is not self-explanatory, document it in the relevant test or add a short explanation nearby.

## Expected Output and Snapshot Files

Fixtures may also contain expected output used for comparisons.

For example:

```text
tests/fixtures/
└── parser/
    ├── input.json
    └── expected_output.json
```

A test can read both files and compare the parser's actual result with the expected result.

Keep expected output files distinct from input files when this makes their roles clearer.

If a fixture represents an expected result, changes to it should be reviewed carefully. Updating an expected result merely to make a failing test pass can hide an actual regression.

## What Does Not Belong Here?

The `tests/fixtures/` directory should contain test data, not general-purpose Rust code.

In particular, avoid placing the following here:

- Rust structures and data types used by tests.
- Functions for constructing test objects.
- Shared setup and teardown logic.
- Assertion helpers.
- Database connection or initialization logic.
- Temporary output generated during test execution.

These responsibilities belong in the appropriate test modules or in `tests/common/`.

## General Principle

The `tests/fixtures/` directory provides a central location for reusable, predefined test data.

Keeping fixtures separate from test code improves readability, makes test scenarios easier to understand, reduces duplication, and allows multiple integration tests to use the same input data consistently.

**In short:**

- `tests/` contains the integration tests.
- `tests/common/` contains reusable Rust test-support code.
- `tests/fixtures/` contains reusable test data files.
