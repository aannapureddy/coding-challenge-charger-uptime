## Testing Strategy

### Unit Tests
- Parser: valid/invalid headers, token parsing, boolean `up`, `end > start`.
- Intervals: overlapping, adjacent, disjoint, zero-length filtered.
- Uptime: single charger 100%, gaps count as downtime, floor rounding.

### Integration Test
`tests/cli_tests.rs` runs the compiled binary against `fixtures/input_1.txt` and `fixtures/input_2.txt` and compares stdout to `*_expected_stdout.txt`.

### Commands
```bash
cargo test
# Run only integration test
cargo test --test cli_tests
```


