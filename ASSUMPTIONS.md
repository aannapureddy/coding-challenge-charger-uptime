## Assumptions and Clarifications

- IDs and times adhere to specified integer ranges; parsing fails otherwise.
- A station line must contain at least one charger.
- Intervals are half-open `[start, end)`; zero-length or `end <= start` are invalid and ignored in merges.
- Denominator uses union of per-charger spans to account for downtime during gaps between a charger's reports.
- If a station has no reporting span (no reports for any of its chargers), the program treats this as an error condition.
- Output is sorted by `StationID` ascending.
- Uptime percent is floored to an integer in `[0, 100]`.


