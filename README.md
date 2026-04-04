# Sentinel

> A compiler-agnostic AI-assisted build copilot that wraps your compiler to diagnose and fix errors automatically.

Sentinel is a compiler-agnostic CLI wrapper built in Rust that uses AI to diagnose and fix build errors. Run your normal compiler through Sentinel, and it will analyze warnings and errors, apply targeted fixes, recompile automatically, and roll back changes if a fix fails.

## Status

🚧 **Early Development** — This project is just starting. Nothing is stable yet.

## Features

- **Compiler Agnostic** — Works in front of standard compilers such as GCC, G++, and Clang.
- **AI-Powered Diagnostics** — Uses AI to diagnose compiler-reported problems and generate narrowly scoped fixes.
- **Self-Healing Loop** — Automatically recompiles after each applied fix to validate changes.
- **Safe by Default** — Restores original source state if a suggested fix fails validation or makes things worse.
- **Rust-Fast** — Built in Rust for speed and reliability.

## Quick Start

Run your normal compiler through Sentinel:

```bash
sentinel g++ main.cpp
sentinel gcc main.c -o main
sentinel clang++ src/main.cpp -o app
```

Sentinel executes the compiler, captures the output, and if errors occur, it begins the self-healing loop.

## How It Works

Sentinel acts as a smart wrapper around your existing toolchain:

1. **Invocation**: You invoke the compiler through Sentinel (e.g., `sentinel gcc ...`).
2. **Capture**: Sentinel executes the command and captures `stdout` and `stderr`.
3. **Diagnosis**: If the build fails, Sentinel sends relevant diagnostics and source context to the AI.
4. **Fix**: Sentinel applies a small, targeted fix to the source file.
5. **Validation**: Sentinel triggers a recompile automatically.
6. **Rollback**: If the fix fails validation, Sentinel restores the file to its previous state.

The loop continues until the build succeeds or a retry limit is reached.

## Safety Model

Sentinel is built on a "Trust but Verify" model:

- **Snapshots**: Sentinel snapshots files before any modification.
- **Targeted Edits**: Fixes are small and localized rather than broad refactors.
- **Continuous Validation**: Every applied fix is immediately validated by recompilation.
- **Automatic Rollback**: If a fix fails or introduces new errors, Sentinel reverts to the last known good state.

## Scope

Sentinel is designed to handle:

- Syntax mistakes
- Misspellings and symbol-name issues
- Common compiler-reported warnings and errors
- Minor, local logic issues where compiler context is sufficient

## Roadmap

- [ ] Compiler wrapper execution and command proxying
- [ ] Diagnostic capture and parsing (GCC/Clang)
- [ ] AI integration for targeted fix generation
- [ ] Self-healing recompile loop logic
- [ ] Automatic snapshot and rollback safety system
- [ ] Configuration file support (`Sentinel.toml`)
- [ ] Interactive fix review mode

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome once the project reaches a stable state. Check back later for contribution guidelines.
