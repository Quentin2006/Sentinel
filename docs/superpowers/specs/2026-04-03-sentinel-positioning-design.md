# Sentinel Positioning and README Design

## Summary

Sentinel should be positioned as a **compiler-agnostic AI-assisted build copilot** implemented as a **compiler wrapper**, not as a pure stdin consumer. The README should center on the wrapper workflow (`sentinel g++ main.cpp`) and explain Sentinel's self-healing recompilation loop, targeted fix application, and safe rollback behavior.

## Product Framing

Sentinel wraps a user's normal compiler command, captures build output, and uses AI to diagnose and resolve common build failures with minimal disruption.

Core framing:

- **Compiler-agnostic**: works in front of standard compilers such as GCC, G++, and Clang
- **AI-assisted**: uses AI to diagnose compiler-reported problems and generate narrowly scoped fixes
- **Self-healing**: recompiles automatically after each applied fix
- **Safe by default**: restores original source state if a suggested fix fails validation

Recommended positioning language:

> Sentinel is a compiler-agnostic CLI wrapper built in Rust that uses AI to diagnose and fix build errors. Run your normal compiler through Sentinel, and it will analyze warnings and errors, apply targeted fixes, recompile automatically, and roll back changes if a fix fails.

## Primary UX

The README should present Sentinel as a wrapper-first experience.

Primary examples:

```bash
sentinel g++ main.cpp
sentinel gcc main.c -o main
sentinel clang++ src/main.cpp -o app
```

The documentation should avoid framing Sentinel as a pipe-first tool. Pipe mode is out of scope for the current story.

## README Structure

The README should be reorganized around the following sections:

1. **What Sentinel is**
   - Short tagline
   - One-paragraph wrapper-based overview
2. **Why use it**
   - Compiler agnostic
   - AI diagnosis
   - Automatic fixes
   - Self-healing recompilation loop
   - Safe rollback
3. **Quick start**
   - Wrapper-based commands only
4. **How it works**
   - Sentinel launches the compiler
   - Captures stdout/stderr
   - Sends relevant diagnostics and source context to the AI
   - Applies a targeted fix
   - Recompiles automatically
   - Reverts if validation fails
5. **Safety model**
   - Snapshot before editing
   - Incremental changes
   - Recompile after every fix
   - Restore original source on failure
6. **Scope**
   - Syntax mistakes
   - Misspellings and symbol-name issues
   - Compiler-reported warnings/errors
   - Minor fixable logic issues
7. **Status / roadmap**
   - Early development
   - Planned compiler support, configuration, and review features
8. **License**
   - MIT

## Runtime Behavior

The README should describe Sentinel's behavior as:

1. The user invokes the compiler through Sentinel.
2. Sentinel executes the compiler and captures stdout/stderr.
3. If warnings or errors are emitted, Sentinel sends relevant diagnostics plus source context to the AI.
4. Sentinel applies a small, targeted fix to source files.
5. Sentinel recompiles automatically.
6. The loop continues until the build succeeds, Sentinel reaches a retry limit, or Sentinel decides no safe next fix is available.
7. If a fix makes things worse or fails validation, Sentinel restores the original source state.

## Trust Model

The README should emphasize trust boundaries clearly:

- Sentinel snapshots files before modifying them.
- Fixes should be small and targeted rather than broad refactors.
- Every applied fix should be validated by recompilation.
- Failed fixes should trigger automatic rollback.
- Since the project is early-stage, the README should explicitly set expectations around maturity and stability.

## Scope and Claims

The README should avoid over-claiming. Acceptable claim boundaries:

- Good for syntax mistakes
- Good for misspellings and symbol-name issues
- Good for compiler-reported warnings and errors
- May help with minor, local logic issues when compiler context is sufficient

The README should avoid implying broad autonomous program repair or deep semantic correctness guarantees.

## Licensing Decision

The project should use the **MIT License** in the README and related project metadata.

## Out of Scope

- Pipe-first or stdin-first product framing
- Broad claims about fixing arbitrary logic bugs
- Detailed implementation internals beyond what is needed to explain the wrapper workflow

## Implementation Consequences

If documentation and future planning follow this design, they should assume:

- Sentinel is invoked in place of the compiler command
- Compiler invocation and recompilation are first-class responsibilities
- Rollback and validation behavior are part of the core product story, not optional extras
- Future plans should prioritize wrapper execution, diagnostic capture, fix application, retry logic, and rollback safety
