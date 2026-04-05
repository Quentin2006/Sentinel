# Sentinel Test Files

Test C++ programs for validating Sentinel's debugging skill.

## Directory Structure

- `test/` - Test source files (this directory)
- `test-backup/` - Backup storage for file snapshots during fix attempts
- `tests/` - Rust integration tests (future)

## Valid Programs (should compile cleanly)

| File | Description |
|------|-------------|
| `valid_hello.cpp` | Simple hello world program |
| `valid_math.cpp` | Basic math functions |

## Broken Programs (Sentinel should fix)

| File | Error Type | Description |
|------|------------|-------------|
| `broken_syntax.cpp` | Syntax | Missing semicolons |
| `broken_type.cpp` | Type mismatch | Invalid type assignments |
| `broken_undeclared.cpp` | Undeclared | Typos and missing declarations |
| `broken_braces.cpp` | Syntax | Unbalanced braces |
| `broken_include.cpp` | Include | Misspelled header name |

## Expected Behavior

Sentinel runs **silently**. You should see no compiler output (no warnings, no errors, no diagnostics).

**Valid programs**: Compile silently, produce binary.
**Broken programs**: Sentinel silently fixes and compiles, or silently fails.

## Usage

Test a valid program (should compile silently):
```bash
sentinel g++ test/valid_hello.cpp -o test/valid_hello
# No output expected - just produces the binary
```

Test a broken program (Sentinel fixes silently):
```bash
sentinel g++ test/broken_syntax.cpp -o test/broken_syntax
# No output expected - Sentinel fixes the errors internally
```

## Notes

These test files are designed to exercise the debugging skill at `src/skills/debugging.md`.
Each broken file targets a specific error category that the skill documents.
