# Sentinel

A Rust command-line wrapper that captures build diagnostics, asks OpenCode to attempt a repair, and reruns the original build command to check its exit status.

Sentinel works with compiler and build commands such as GCC, Clang, and Make. It delegates source edits to OpenCode; it does not implement its own language parser or guarantee that a proposed repair is correct.

## Requirements

- Rust with support for the 2024 edition and Cargo.
- [OpenCode](https://opencode.ai/) installed as `opencode`, with a configured model provider.
- The compiler or build tool used by your project.

The repair agent can edit files in the working directory and may incur model-provider charges. Use a version-controlled checkout and review its changes before keeping them.

## Install

```sh
git clone https://github.com/Quentin2006/Sentinel.git
cd Sentinel
cargo install --path .
```

Cargo installs `ai-sentinel` into its binary directory (usually `~/.cargo/bin`). The debugging instructions are embedded in the executable, so installation does not depend on a particular checkout path.

## Usage

Run from the project you want to build:

```sh
ai-sentinel gcc main.c -Wall -o app
ai-sentinel clang++ src/main.cpp -o app
ai-sentinel make
ai-sentinel --verbose --model YOUR_MODEL_ID make
ai-sentinel --level warning make
```

Sentinel options precede the compiler or build command. Everything following that command is passed to it as arguments, including compiler flags.

Repair scopes:

- `error` (default): request a repair when the build exits unsuccessfully. Successful builds that write warnings to stderr do not trigger a repair.
- `warning`: also request a repair when a successful build writes diagnostics to stderr; ask the agent to address errors and warnings.
- `logic`: use the same trigger as `warning`, but also ask the agent to inspect logic issues. This is a prompt scope, not a static-analysis guarantee.

## Execution flow

1. Run the supplied build command and capture its status, stdout, and stderr.
2. If repair is needed, invoke OpenCode once with the diagnostics, selected model, repair scope, and embedded debugging instructions.
3. If OpenCode exits successfully, independently rerun the original build command.
4. Return success only if that verification build exits successfully. A failing repair agent or verification build returns a nonzero status.

OpenCode may perform its own iterations during that single invocation. Sentinel does not run an unbounded outer repair loop. A successful verification establishes build success, not functional correctness or warning-free output. Run your project's tests and inspect the diff separately.

## Development checks

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

The Unix integration tests use isolated fake compiler and agent executables. They cover successful builds with stderr output, failed builds with stdout-only or no diagnostics, compiler flag forwarding, repair-agent failures, verification failures, and warning-scope triggering. They do not call a real model or measure repair accuracy.

## Source map

- `src/main.rs`: CLI arguments, build execution, repair prompt, and verification.
- `src/msg.rs`: terminal progress and diagnostic rendering.
- `src/skills/debugging.md`: embedded repair instructions.
- `tests/cli.rs`: deterministic process-level regression tests.

## License

[MIT](LICENSE).
