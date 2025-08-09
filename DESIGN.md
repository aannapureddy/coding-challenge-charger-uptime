## Design

### Data Model
- `StationId(u32)`, `ChargerId(u32)` newtypes
- `Interval { start: u64, end: u64 }` (half-open)
- `ChargerReport { charger: ChargerId, interval: Interval, up: bool }`
- `Station { id: StationId, chargers: Vec<ChargerId> }`

### Parsing
Input has two sections: `[Stations]` and `[Charger Availability Reports]`.
- Stations: `<StationID> <ChargerID...>` (at least one charger per station)
- Reports: `<ChargerID> <start_nanos> <end_nanos> <up true/false>`
- Validation: numeric parsing, `end > start`, and `up` is `true|false`.
- Blank lines are skipped. Any malformed line yields an error.

### Interval Semantics
Intervals are treated as half-open `[start, end)`. This avoids double-counting shared endpoints and allows merging adjacent intervals safely.

### Merging
`merge_intervals`:
1. Filter invalid/zero-length intervals.
2. Sort by `start`.
3. Sweep and merge overlapping or adjacent intervals.

### Uptime Computation
For each station:
- Denominator: For each charger at the station, compute its overall reporting span `[min(start), max(end))`. The union of these spans is the station's reporting window, so gaps in a charger's reports count as downtime.
- Numerator: Union of all intervals where any charger reported `up == true`.
- Percentage: `floor(100 * up / reported)`; error if reported is zero.

### Complexity
Let `N` be number of report intervals for a station.
- Sorting dominates: `O(N log N)` time, `O(N)` memory.


