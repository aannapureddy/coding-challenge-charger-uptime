## Submission Package

Include the following in a zip and email as requested:

- Source code: `src/`, `Cargo.toml`, `Cargo.lock`
- Tests: `tests/`, `fixtures/`
- Docs: `README.md`, `DESIGN.md`, `ASSUMPTIONS.md`, `TESTING.md`
- Original prompt: `challenge_original/README.md`

### Build & Run Instructions

1. Ensure Rust stable is installed.
2. Build: `cargo build`
3. Test: `cargo test`
4. Run: `cargo run -- <path/to/input>`

### Create Zip
```bash
tar -czf submission.zip src Cargo.toml Cargo.lock README.md DESIGN.md ASSUMPTIONS.md TESTING.md SUBMISSION.md fixtures challenge_original
```


