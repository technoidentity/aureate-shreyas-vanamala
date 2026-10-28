# Rustlings Debugging Exercises

This directory contains Rust debugging exercises. Each topic folder groups
small programs with compiler errors, incorrect behavior, failing tests, or
Clippy warnings to diagnose and fix.

The folders are organized by Rust topic, such as variables, vectors, ownership,
error handling, iterators, smart pointers, threads, and macros. The `quizzes`
folder combines several topics in one exercise.

## Run the exercises

From the project root, run all exercise tests:

```bash
cargo test --bins
```

To run one exercise, replace the bin name with its file name without `.rs`:

```bash
cargo test --bin vecs2
```

To check the Clippy exercises and other lint warnings:

```bash
cargo clippy --bins -- -D warnings
```

Read the comments and tests in each `.rs` file, identify the bug, make the
smallest correct fix, and rerun the relevant command.
