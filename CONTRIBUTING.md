# Preparing learning changes for review

These practices capture feedback from [PR #1](https://github.com/technoidentity/aureate-shreyas-vanamala/pull/1).

## Commits

Keep each commit focused and its subject specific and complete. Prefer descriptions such as `Add ladder movement tests` or `Fix hallo_cargo executable target`; avoid subjects such as `git` or `Prac:`. Use imperative wording consistently. Review `git diff --cached` before committing so unrelated or accidental files do not enter the commit. Do not rewrite published history merely to improve older subjects.

## Source and learning notes

Files under `src/bin/` must be complete, compilable programs with the required definitions and a `main` function. Cargo packages must have a source target, such as `src/main.rs`, `src/lib.rs`, or an explicit target in `Cargo.toml`. Keep incomplete study snippets in Markdown notes with fenced code blocks, or turn them into complete runnable examples.

Prefer Markdown for new learning notes so reviewers can read their diffs. Existing DOCX notes may remain; link them or a shared document from the PR when relevant. Do not claim an existing document has been converted unless it actually has.

## Verification

Run these from the repository root before requesting review:

```bash
cargo check --locked --all-targets --manifest-path hallo_cargo/Cargo.toml
cargo check --locked --all-targets --manifest-path Rust_practice/Cargo.toml
cargo check --locked --all-targets --manifest-path Rust_practice/Rust_Programs/Cargo.toml
cargo check --locked --all-targets --manifest-path Rust_practice/Rust_Programs/snakes_and_ladders/Cargo.toml
cargo fmt --check --manifest-path Rust_practice/Rust_Programs/snakes_and_ladders/Cargo.toml
cargo test --locked --manifest-path Rust_practice/Rust_Programs/snakes_and_ladders/Cargo.toml
cargo build --release --locked --manifest-path Rust_practice/Rust_Programs/snakes_and_ladders/Cargo.toml
```

The CI package matrix checks these four packages. Rustlings is a separate learning harness; validate its exercises using Rustlings rather than assuming every intentionally incomplete exercise is a normal application target.

## Pull requests

For weekly submissions, follow the existing title style: `Learning Progress: Week N - Topics`. Use the actual week and describe the final scope. Compare the source branch against the destination branch and review all changes since the preceding merge.

Update the title and description when new work is added. Explain what changed, why, how it was verified, and any remaining limitations. Report exact commands and outcomes; distinguish automated checks from manual observations and untested work. Link learning notes when relevant. A passing test command with zero tests is not gameplay coverage.

PR checks run when the destination is `Shreyas` or `develop`. A local check does not prove that a hosted workflow passed; inspect the PR checks after pushing.
