## Charger Uptime Solution

This is a Rust implementation of the Electric Era charger uptime challenge. It reads an input file describing stations, chargers, and charger availability reports, and prints each station's uptime percentage to stdout.

### Requirements
- Rust (stable). This repo was built and tested with stable toolchain on Linux/macOS.

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

### Notes
- Interval semantics are half-open `[start, end)`. Adjacent intervals merge without double-counting.
- Denominator is the union of each charger's overall reporting span at a station; gaps inside a span count as downtime.
- Numerator is the union of all intervals marked `up == true` across chargers at a station.
- Uptime is floored to an integer percent in `[0,100]`.


