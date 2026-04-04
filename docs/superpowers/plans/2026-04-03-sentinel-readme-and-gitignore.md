# Sentinel README and Gitignore Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite Sentinel's README around the approved wrapper-first product design and add a root `.gitignore` that keeps documentation tracked while ignoring standard Rust/build/temp artifacts.

**Architecture:** This change is documentation-and-repo-hygiene only. The README becomes the canonical product story for Sentinel as a compiler wrapper with a self-healing recompilation loop and rollback safety model. The `.gitignore` should be minimal: preserve tracked `docs/` content while excluding standard Rust build outputs, editor junk, temp files, and generated documentation artifacts.

**Tech Stack:** Markdown, gitignore syntax, Rust project conventions

---

## File Structure

### Files to modify
- `README.md` — Replace stdin/pipe-first messaging with wrapper-first messaging, update examples, flow diagrams, scope, safety model, roadmap, and MIT license text.

### Files to create
- `.gitignore` — Ignore standard Rust build artifacts and local/generated junk while keeping `docs/` tracked.
- `LICENSE` — Add standard MIT license text so the README's MIT claim points to a real license file.

### Files to reference
- `docs/superpowers/specs/2026-04-03-sentinel-positioning-design.md` — Approved source of truth for product framing.

### Constraints
- Do **not** ignore the `docs/` directory wholesale.
- Do **not** reintroduce pipe-first wording in the README.
- Do **not** over-claim autonomous bug fixing beyond the approved scope.
- Do **not** add Cargo-specific metadata files unless required by the user.
- Repository is currently **not** a git repo, so commit steps should be documented but skipped unless git is initialized later.

---

## Chunk 1: Rewrite README around wrapper-first positioning

### Task 1: Audit README sections against approved spec

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-04-03-sentinel-positioning-design.md`

- [ ] **Step 1: Read the approved spec and current README side by side**

Read:
- `docs/superpowers/specs/2026-04-03-sentinel-positioning-design.md`
- `README.md`

Expected findings:
- Current README still contains pipe/stdin language
- Spec requires wrapper-first invocation and rollback-centered trust model

- [ ] **Step 2: Write down exact README sections that must change**

Capture a checklist covering:
- tagline
- feature bullets
- quick-start examples
- how-it-works diagram
- numbered workflow
- roadmap item mentioning stdin parser
- license section

Expected result: a concrete edit list with no ambiguous “update docs” steps.

### Task 2: Rewrite README copy and examples

**Files:**
- Modify: `README.md`

- [ ] **Step 3: Replace the opening summary with wrapper-first positioning**

Update the top description so it says Sentinel is a compiler-agnostic CLI wrapper built in Rust that uses AI to diagnose and fix build errors.

Minimum content to include:
- wrapper-based wording
- AI-assisted diagnosis/fixes
- build-error framing

- [ ] **Step 4: Replace pipe/stdin feature language with wrapper/safety language**

Update the feature list to include:
- compiler agnostic
- AI-powered diagnostics
- automatic code fixes
- self-healing recompilation loop
- safe rollback on failed fixes

Remove any feature phrased as `gcc ... 2>&1 | sentinel`.

- [ ] **Step 5: Replace quick-start examples with wrapper commands**

Use commands like:

```bash
sentinel g++ main.cpp
sentinel gcc main.c -o main
sentinel clang++ src/main.cpp -o app
```

Expected result: README examples all reinforce wrapper-first invocation.

- [ ] **Step 6: Rewrite the “How It Works” section and diagram**

The flow must show:
1. user invokes compiler through Sentinel
2. Sentinel launches compiler
3. Sentinel captures stdout/stderr
4. AI analyzes relevant diagnostics
5. Sentinel applies a targeted fix
6. Sentinel recompiles
7. Sentinel rolls back if validation fails

Expected result: no stdin-consumer diagram remains.

- [ ] **Step 7: Add a dedicated safety model section**

Create a section that explicitly states:
- snapshot before edits
- small targeted changes
- recompile after each fix
- rollback on failed validation

Expected result: trust model is first-class, not implied.

- [ ] **Step 8: Add or refine a scope section with credible claims**

Allowed scope language:
- syntax mistakes
- misspellings / symbol-name issues
- compiler-reported warnings and errors
- minor fixable logic issues

Expected result: no broad “fixes all bugs” claims.

- [ ] **Step 9: Update roadmap and license wording**

Update roadmap items to match wrapper execution work, for example:
- compiler wrapper execution
- diagnostic capture/parsing
- fix application
- retry loop
- rollback safety

Update license section to say MIT and point to `LICENSE`.

### Task 3: Verify README changes

**Files:**
- Modify: `README.md`

- [ ] **Step 10: Run targeted content checks against README**

Run:

```bash
grep -nE 'pipe|stdin|2>&1 \| sentinel' README.md
grep -nE 'sentinel (g\+\+|gcc|clang\+\+)' README.md
grep -n 'MIT' README.md
```

Expected:
- first command finds no outdated architecture wording (except if mentioning pipe mode explicitly as out of scope, which is not preferred)
- second command finds wrapper examples
- third command finds MIT reference

- [ ] **Step 11: Read the final README end-to-end**

Verify manually:
- wrapper-first story is consistent
- examples match the story
- safety model is present
- no contradictory pipe-first language remains

- [ ] **Step 12: Record verification note**

Write a brief note in implementation output or notepad summarizing:
- outdated terms removed
- wrapper examples present
- MIT reference present

### Chunk 1 Review

- [ ] **Step 13: Dispatch plan review for Chunk 1 execution readiness**

Reviewer should verify that Chunk 1 is specific enough to execute without guessing and that README verification is concrete.

---

## Chunk 2: Add repository hygiene files

### Task 4: Create minimal `.gitignore`

**Files:**
- Create: `.gitignore`

- [ ] **Step 14: Create `.gitignore` with standard Rust/build ignores**

Include entries for:

```gitignore
/target/
**/*.rs.bk
*.log
*.tmp
*.swp
*.swo
.DS_Store
```

If desired, include common editor junk such as `.idea/` and `.vscode/`.

Do **not** include `docs/`.

- [ ] **Step 15: Add generated-doc/temp ignore patterns only if they are truly local artifacts**

Acceptable examples:

```gitignore
docs/**/*.tmp
docs/**/.DS_Store
```

Do **not** ignore the spec/plan markdown files.

- [ ] **Step 16: Verify `.gitignore` does not suppress tracked docs**

Run:

```bash
grep -n '^docs/?$' .gitignore
grep -n '^docs/\*\*$' .gitignore
grep -n '^/target/$' .gitignore
```

Expected:
- no lines that ignore the whole docs tree
- target is ignored

### Task 5: Add MIT license file

**Files:**
- Create: `LICENSE`

- [ ] **Step 17: Create `LICENSE` with standard MIT text**

Use the canonical MIT license template and set copyright holder/year.

If the repository has no explicit owner information, leave a placeholder requiring user follow-up, or ask before filling in a legal name if necessary.

- [ ] **Step 18: Ensure README and LICENSE agree**

Read:
- `README.md`
- `LICENSE`

Expected result: README says MIT and the LICENSE file exists with matching license name.

### Task 6: Final verification

**Files:**
- Read: `README.md`
- Read: `.gitignore`
- Read: `LICENSE`

- [ ] **Step 19: Run final cross-file verification**

Run:

```bash
test -f README.md && echo README_OK
test -f .gitignore && echo GITIGNORE_OK
test -f LICENSE && echo LICENSE_OK
grep -n 'MIT' README.md LICENSE
```

Expected:
- all three files exist
- MIT appears in README and LICENSE

- [ ] **Step 20: Run diagnostics/readability pass**

Use file reads/manual review to verify:
- markdown formatting is clean
- `.gitignore` patterns are minimal and not over-broad
- LICENSE text is present and complete

- [ ] **Step 21: Document git limitation**

Note explicitly in completion output:
- git commits were not created because `/home/qscheetz/Documents/Sentinel` is not a git repository

### Chunk 2 Review

- [ ] **Step 22: Dispatch plan review for Chunk 2 execution readiness**

Reviewer should verify that `.gitignore` scope is safe and that MIT licensing work is sufficiently specified.

---

## Execution Notes

- Use TDD where code behavior exists; for this docs/repo-hygiene task, verification is content-based rather than test-based.
- Keep edits focused; do not introduce unrelated project scaffolding.
- If implementation begins before git is initialized, skip commit steps and report that limitation clearly.
- If the user wants the MIT `LICENSE` file to contain a specific legal name, confirm the holder string before finalizing.

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-04-03-sentinel-readme-and-gitignore.md`. Ready to execute?
