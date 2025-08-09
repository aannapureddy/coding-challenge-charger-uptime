use crate::interval::merge_intervals;
use crate::types::{ChargerId, ChargerReport, Interval, Station, StationId};

#[derive(thiserror::Error, Debug)]
pub enum UptimeError {
    #[error("no reporting intervals for station {0:?}")]
    NoReporting(StationId),
}

pub fn compute_station_uptime(
    stations: &[Station],
    reports: &[ChargerReport],
) -> Result<Vec<(StationId, u8)>, UptimeError> {
    // Index reports by charger for efficient lookup per station.
    let mut reports_by_charger: std::collections::BTreeMap<ChargerId, Vec<&ChargerReport>> =
        std::collections::BTreeMap::new();
    for r in reports {
        reports_by_charger.entry(r.charger).or_default().push(r);
    }

    let mut results: Vec<(StationId, u8)> = Vec::with_capacity(stations.len());
    for station in stations {
        // Denominator strategy:
        // For each charger present at this station, find its overall reporting SPAN
        // defined as [min(start), max(end)). Gaps inside this span count as downtime,
        // per the prompt. The station denominator is the UNION of each charger's span.
        let mut reporting_spans: Vec<Interval> = Vec::new();

        // Numerator strategy:
        // Union of all intervals marked up == true across all chargers at the station.
        let mut all_up: Vec<Interval> = Vec::new();

        for charger in &station.chargers {
            if let Some(list) = reports_by_charger.get(charger) {
                let mut min_start: Option<u64> = None;
                let mut max_end: Option<u64> = None;

                for r in list.iter().copied() {
                    // Track charger span for denominator
                    min_start = Some(match min_start {
                        Some(s) => s.min(r.interval.start),
                        None => r.interval.start,
                    });
                    max_end = Some(match max_end {
                        Some(e) => e.max(r.interval.end),
                        None => r.interval.end,
                    });

                    // Track up intervals for numerator
                    if r.up {
                        all_up.push(r.interval);
                    }
                }

                if let (Some(s), Some(e)) = (min_start, max_end) {
                    if e > s {
                        reporting_spans.push(Interval { start: s, end: e });
                    }
                }
            }
        }

        // The denominator is the total time covered by the union of charger spans
        let merged_reporting = merge_intervals(&mut reporting_spans);
        let merged_up = merge_intervals(&mut all_up);

        let reported_duration: u128 = merged_reporting
            .iter()
            .map(|iv| (iv.end - iv.start) as u128)
            .sum();
        if reported_duration == 0 {
            // Policy: surface an error if no charger reported for this station
            return Err(UptimeError::NoReporting(station.id));
        }
        let up_duration: u128 = merged_up.iter().map(|iv| (iv.end - iv.start) as u128).sum();

        let percent = ((up_duration.saturating_mul(100)) / reported_duration) as u8; // floor division
        results.push((station.id, percent));
    }

    // Output must be sorted by station id ascending
    results.sort_by_key(|(sid, _)| *sid);
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChargerId, Interval, Station};

    #[test]
    fn single_charger_full_uptime() {
        let stations = vec![Station {
            id: StationId(1),
            chargers: vec![ChargerId(10)],
        }];
        let reports = vec![ChargerReport {
            charger: ChargerId(10),
            interval: Interval { start: 0, end: 100 },
            up: true,
        }];
        let res = compute_station_uptime(&stations, &reports).unwrap();
        assert_eq!(res, vec![(StationId(1), 100)]);
    }

    #[test]
    fn gaps_count_as_downtime() {
        let stations = vec![Station {
            id: StationId(1),
            chargers: vec![ChargerId(10)],
        }];
        let reports = vec![
            ChargerReport {
                charger: ChargerId(10),
                interval: Interval { start: 0, end: 50 },
                up: true,
            },
            // gap 50..70 counts as down
            ChargerReport {
                charger: ChargerId(10),
                interval: Interval {
                    start: 70,
                    end: 100,
                },
                up: true,
            },
        ];
        let res = compute_station_uptime(&stations, &reports).unwrap();
        assert_eq!(res, vec![(StationId(1), 80)]); // up=80/100
    }

    #[test]
    fn multiple_chargers_union_denominator_and_up() {
        // Station with two chargers; their spans overlap partially.
        // Reports:
        // C1: [0,50) up; [50,100) down (no report)
        // C2: [30,120) up
        // Denominator = union of spans: C1 span [0,50), C2 span [30,120) -> [0,120)
        // Up union = [0,50) U [30,120) -> [0,120) => 100%
        let stations = vec![Station {
            id: StationId(1),
            chargers: vec![ChargerId(10), ChargerId(11)],
        }];
        let reports = vec![
            ChargerReport {
                charger: ChargerId(10),
                interval: Interval { start: 0, end: 50 },
                up: true,
            },
            ChargerReport {
                charger: ChargerId(11),
                interval: Interval {
                    start: 30,
                    end: 120,
                },
                up: true,
            },
        ];
        let res = compute_station_uptime(&stations, &reports).unwrap();
        assert_eq!(res, vec![(StationId(1), 100)]);
    }

    #[test]
    fn rounding_down_behavior() {
        // up=99, reported=100 -> 99%
        let stations = vec![Station {
            id: StationId(1),
            chargers: vec![ChargerId(10)],
        }];
        let reports = vec![
            ChargerReport {
                charger: ChargerId(10),
                interval: Interval { start: 0, end: 99 },
                up: true,
            },
            ChargerReport {
                charger: ChargerId(10),
                interval: Interval {
                    start: 99,
                    end: 100,
                },
                up: false,
            },
        ];
        let res = compute_station_uptime(&stations, &reports).unwrap();
        assert_eq!(res, vec![(StationId(1), 99)]);
    }
}
