# AGENTS.md — toon_rust (tr)

> Guidelines for AI coding agents working in this Rust codebase.

---

## RULE NUMBER 1: NO FILE DELETION

**YOU ARE NEVER ALLOWED TO DELETE A FILE WITHOUT EXPRESS PERMISSION.** Even a new file that you yourself created, such as a test code file. You have a horrible track record of deleting critically important files or otherwise throwing away tons of expensive work. As a result, you have permanently lost any and all rights to determine that a file or folder should be deleted.

**YOU MUST ALWAYS ASK AND RECEIVE CLEAR, WRITTEN PERMISSION BEFORE EVER DELETING A FILE OR FOLDER OF ANY KIND.**

---

## Irreversible Git & Filesystem Actions — DO NOT EVER BREAK GLASS

1. **Absolutely forbidden commands:** `git reset --hard`, `git clean -fd`, `rm -rf`, or any command that can delete or overwrite code/data must never be run unless the user explicitly provides the exact command and states, in the same message, that they understand and want the irreversible consequences.
2. **No guessing:** If there is any uncertainty about what a command might delete or overwrite, stop immediately and ask the user for specific approval. "I think it's safe" is never acceptable.
3. **Safer alternatives first:** When cleanup or rollbacks are needed, request permission to use non-destructive options (`git status`, `git diff`, `git stash`, copying to backups) before ever considering a destructive command.
4. **Mandatory explicit plan:** Even after explicit user authorization, restate the command verbatim, list exactly what will be affected, and wait for a confirmation that your understanding is correct. Only then may you execute it—if anything remains ambiguous, refuse and escalate.
5. **Document the confirmation:** When running any approved destructive command, record (in the session notes / final response) the exact user text that authorized it, the command actually run, and the execution time. If that record is absent, the operation did not happen.

---

## Toolchain: Rust & Cargo (Beads-Rust Parity)

We only use **Cargo** in this project, NEVER any other package manager.

- **Edition:** Rust 2024 (nightly required — see `rust-toolchain.toml`)
- **Dependency versions:** Explicit versions for stability
- **Configuration:** Cargo.toml only
- **Unsafe code:** Forbidden (`#![forbid(unsafe_code)]` via crate lints)

### Release Profile

The release build optimizes for binary size:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
panic = "abort"     # Smaller binary, no unwinding overhead
strip = true        # Remove debug symbols
```

---

## Code Editing Discipline

### No Script-Based Changes

**NEVER** run a script that processes/changes code files in this repo. Brittle regex-based transformations create far more problems than they solve.

- **Always make code changes manually**, even when there are many instances
- For many simple changes: use parallel subagents
- For subtle/complex changes: do them methodically yourself

### No File Proliferation

If you want to change something or add a feature, **revise existing code files in place**.

**NEVER** create variations like:
- `mainV2.rs`
- `main_improved.rs`
- `main_enhanced.rs`

New files are reserved for **genuinely new functionality** that makes zero sense to include in any existing file. The bar for creating new files is **incredibly high**.

---

## Project Semantics (toon_rust / tr)

This project is a Rust port of the TOON reference implementation.

- **Spec-first port:** Extract spec from legacy → implement from spec → never translate line-by-line.
- **TOON format fidelity:** Output must conform to the official TOON spec (v3.0) and match reference behavior for core paths.
- **JSON model parity:** Encode/decode must preserve JSON primitives, arrays, and objects losslessly.
- **Streaming-first:** Encode and decode should support streaming APIs where possible (line/event iterators).
- **CLI parity:** `tr` should mirror the `toon` CLI flags and behavior (auto-detect encode/decode, stdin/stdout streaming, stats output).

### What We're NOT Porting

- Web docs/playgrounds
- JS monorepo tooling (pnpm, tsdown, automd)
- Editor plugins/VSCode extensions
- Benchmarks site generation (but keep raw fixtures if needed for tests)

(See `PLAN_TO_PORT_TOON_RUST_TO_RUST.md` for full exclusions.)

---

## Output Style

- **Text output** is user-facing and may include color. Avoid verbose debug spew unless `--verbose` is set.
- **JSON output** must be stable and machine-parseable. Do not change JSON shapes without explicit intent and tests.
- **Robot mode (if added):** clean JSON to stdout, diagnostics to stderr.

---

## Compiler Checks (CRITICAL)

**After any substantive code changes, you MUST verify no errors were introduced:**

```bash
# Check for compiler errors and warnings
cargo check --all-targets

# Check for clippy lints (pedantic + nursery are enabled)
cargo clippy --all-targets -- -D warnings

# Verify formatting
cargo fmt --check
```

If you see errors, **carefully understand and resolve each issue**. Read sufficient context to fix them the RIGHT way.

---

## Testing

### Unit Tests

```bash
cargo test
cargo test -- --nocapture
```

### Focused Tests

```bash
cargo test encode
cargo test decode
cargo test cli
```

### Conformance Tests (Planned)

Conformance tests will compare Rust output to legacy/fixture outputs:
1. Run legacy encoder/decoder on fixtures
2. Compare Rust output (TOON + JSON)
3. Validate strict vs non-strict modes

---

## Third-Party Library Usage

If you aren't 100% sure how to use a third-party library, **SEARCH ONLINE** to find the latest documentation and best practices before coding. Prefer primary docs.

---

## Session Start

Always read this file and the spec documents at session start.

---

## Tools Available

- `bd` - Beads task tracking
- `cass` - Session search for context recovery
