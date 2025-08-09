use crate::types::{ChargerReport, Station, StationId};

#[derive(thiserror::Error, Debug)]
pub enum UptimeError {
    #[error("no reporting intervals for station {0:?}")]
    NoReporting(StationId),
}

pub fn compute_station_uptime(
    _stations: &[Station],
    _reports: &[ChargerReport],
) -> Result<Vec<(StationId, u8)>, UptimeError> {
    // Placeholder; to be implemented in Commit 3
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChargerId, Interval, Station};

    #[test]
    #[ignore]
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
    #[ignore]
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
}
