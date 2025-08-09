## Charger Uptime Solution

Find the full code at [https://github.com/aannapureddy/coding-challenge-charger-uptime](https://github.com/aannapureddy/coding-challenge-charger-uptime)

This is a Rust implementation of the Electric Era charger uptime challenge. It reads an input file describing stations, chargers, and charger availability reports, and prints each station's uptime percentage to stdout.

Disclaimer: It goes without saying that AI was used during the process of creating this solution. Steps included: finding the job posting, researching the company and specific role, reviewing the executive team's history, breaking down the challenge requirements, enumerating edge cases, selecting a reasonable algorithm, creating boilerplate code, testing, setting up a GitHub pipeline, and sending the email. This is a good, evolutionary thing.

### Requirements
- Rust (stable). This repo was built and tested with the stable toolchain on Linux/macOS.


### On Tests in Rust
- It is idiomatic in Rust to colocate unit tests inside the source files (within `#[cfg(test)]` modules). These tests are compiled and run only in test builds; they are not included in the final release executable and have no runtime/performance impact in production.
- There is also an additional integration test under the `/tests` directory which exercises the compiled binary against the provided fixtures.

### Build
```bash
cargo build
```

### Run
```bash
cargo run -- <path/to/input_file>

# Examples using included fixtures
cargo run -- fixtures/input_1.txt
cargo run -- fixtures/input_2.txt
```

Output format: one line per station, ascending `StationID`, as `<StationID> <uptime_percent>`.

On invalid input, the program prints `ERROR` to stdout and logs details to stderr, then exits successfully (per prompt).

### Test
```bash
cargo test
```

This runs unit tests (parser, interval merging, uptime math) and an integration test that executes the binary against the sample fixtures in `fixtures/`.

### Continuous Integration
- GitHub Actions runs on `ubuntu-latest` (Linux `x86_64`) to mirror the challenge runtime requirement (Linux `amd64`).
- The workflow builds with `cargo build --locked` and runs `cargo test --locked`.
- See `.github/workflows/ci.yml`.

### Notes
- Interval semantics are half-open `[start, end)`. Adjacent intervals merge without double-counting.
- Denominator is the union of each charger's overall reporting span at a station; gaps inside a span count as downtime.
- Numerator is the union of all intervals marked `up == true` across chargers at a station.
- Uptime is floored to an integer percent in `[0, 100]`.