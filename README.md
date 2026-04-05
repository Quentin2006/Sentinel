# Sentinel

> A compiler-agnostic AI-assisted build copilot that fixes compilation errors automatically.

Sentinel is a compiler-agnostic CLI wrapper built in Rust that uses AI to fix **compilation errors only**. Run your normal compiler through Sentinel, and if the build fails, it will apply minimal fixes to make it compile, revalidate, and roll back if a fix doesn't work.

**Important**: Sentinel fixes compilation errors. It does not refactor, improve, optimize, or change code that already compiles.

## Status

🚧 **Early Development** — This project is just starting. Nothing is stable yet.

## Installation

```bash
cargo install ai-sentinel
```

## Quick Start

Run your normal compiler through Sentinel:

```bash
sentinel g++ main.cpp
sentinel gcc main.c -o main
sentinel clang++ src/main.cpp -o app
```

For development:

```bash
cargo run -- g++ main.cpp
cargo run -- gcc main.c -o main
```

Sentinel runs silently. No compiler output clutters your terminal. It either works, or it doesn't.

## How It Works

Sentinel wraps your compiler and handles errors silently:

1. **Invocation**: Run your compiler through Sentinel (e.g., `sentinel gcc ...`).
2. **Capture**: Sentinel intercepts all compiler output — you see nothing.
3. **Fix**: If compilation fails, Sentinel silently applies minimal fixes.
4. **Validate**: Sentinel recompiles to verify the fix works.
5. **Rollback**: If a fix fails, Sentinel restores the original file.

The loop continues silently until compilation succeeds or Sentinel gives up. No warnings, no errors, no noise.

## Safety Model

Sentinel is built on a "Trust but Verify" model:

- **Snapshots**: Sentinel snapshots files before any modification.
- **Targeted Edits**: Fixes are small and localized rather than broad refactors.
- **Continuous Validation**: Every applied fix is immediately validated by recompilation.
- **Automatic Rollback**: If a fix fails or introduces new errors, Sentinel reverts to the last known good state.

## Scope

Sentinel fixes **compilation errors only**:

- Missing semicolons
- Unbalanced braces/parentheses  
- Typos in identifiers
- Missing includes
- Type mismatches that block compilation

Sentinel does **NOT**:

- Refactor code
- Fix warnings (unless they block compilation)
- Improve code style
- Optimize performance
- Change code that already compiles

## Roadmap

- [x] Compiler wrapper execution and command proxying
- [x] AI integration for targeted fix generation
- [ ] Automatic snapshot and rollback safety system
- [ ] Configuration file support (`Sentinel.toml`)
- [ ] Interactive fix review mode

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome once the project reaches a stable state. Check back later for contribution guidelines.
