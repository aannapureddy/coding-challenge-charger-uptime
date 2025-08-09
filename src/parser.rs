use crate::types::{ChargerId, ChargerReport, Interval, Station, StationId};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

/// Parse the challenge input format:
/// [Stations]\n
/// <station_id> <charger_id> ...\n
/// ...\n
/// \n
/// [Charger Availability Reports]\n
/// <charger_id> <start> <end> <up>\n
pub fn parse_input(input: &str) -> Result<(Vec<Station>, Vec<ChargerReport>), ParseError> {
    enum Section {
        None,
        Stations,
        Reports,
    }

    let mut section = Section::None;
    let mut saw_stations = false;
    let mut saw_reports = false;
    let mut stations: Vec<Station> = Vec::new();
    let mut reports: Vec<ChargerReport> = Vec::new();
    // Track data hygiene constraints while parsing
    let mut seen_station_ids: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    let mut seen_charger_ids: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    let mut known_chargers: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();

    for (line_idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "[Stations]" {
            if saw_stations {
                return Err(ParseError::InvalidFormat(
                    "duplicate [Stations] header".into(),
                ));
            }
            section = Section::Stations;
            saw_stations = true;
            continue;
        }
        if line == "[Charger Availability Reports]" {
            if saw_reports {
                return Err(ParseError::InvalidFormat(
                    "duplicate [Charger Availability Reports] header".into(),
                ));
            }
            section = Section::Reports;
            saw_reports = true;
            continue;
        }

        match section {
            Section::Stations => {
                let mut parts = line.split_whitespace();
                let station_id: u32 = parts
                    .next()
                    .ok_or_else(|| {
                        ParseError::InvalidFormat(format!(
                            "missing station id at line {}",
                            line_idx + 1
                        ))
                    })?
                    .parse()
                    .map_err(|_| {
                        ParseError::InvalidFormat(format!(
                            "invalid station id at line {}",
                            line_idx + 1
                        ))
                    })?;
                if !seen_station_ids.insert(station_id) {
                    return Err(ParseError::InvalidFormat(format!(
                        "duplicate station id {} detected (line {})",
                        station_id,
                        line_idx + 1
                    )));
                }
                let mut chargers: Vec<ChargerId> = Vec::new();
                let mut chargers_in_line: std::collections::BTreeSet<u32> =
                    std::collections::BTreeSet::new();
                for token in parts {
                    let cid: u32 = token.parse().map_err(|_| {
                        ParseError::InvalidFormat(format!(
                            "invalid charger id at line {}",
                            line_idx + 1
                        ))
                    })?;
                    if !chargers_in_line.insert(cid) {
                        return Err(ParseError::InvalidFormat(format!(
                            "duplicate charger id {} on the same station line (line {})",
                            cid,
                            line_idx + 1
                        )));
                    }
                    if !seen_charger_ids.insert(cid) {
                        return Err(ParseError::InvalidFormat(format!(
                            "charger id {} appears under multiple stations (line {})",
                            cid,
                            line_idx + 1
                        )));
                    }
                    known_chargers.insert(cid);
                    chargers.push(ChargerId(cid));
                }
                if chargers.is_empty() {
                    return Err(ParseError::InvalidFormat(format!(
                        "station {} has no chargers (line {})",
                        station_id,
                        line_idx + 1
                    )));
                }
                stations.push(Station {
                    id: StationId(station_id),
                    chargers,
                });
            }
            Section::Reports => {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                if tokens.len() != 4 {
                    return Err(ParseError::InvalidFormat(format!(
                        "invalid report format at line {}",
                        line_idx + 1
                    )));
                }
                let charger: u32 = tokens[0].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!(
                        "invalid charger id at line {}",
                        line_idx + 1
                    ))
                })?;
                if !known_chargers.contains(&charger) {
                    return Err(ParseError::InvalidFormat(format!(
                        "report references unknown charger id {} (line {})",
                        charger,
                        line_idx + 1
                    )));
                }
                let start: u64 = tokens[1].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!(
                        "invalid start time at line {}",
                        line_idx + 1
                    ))
                })?;
                let end: u64 = tokens[2].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("invalid end time at line {}", line_idx + 1))
                })?;
                if end <= start {
                    return Err(ParseError::InvalidFormat(format!(
                        "end must be > start at line {}",
                        line_idx + 1
                    )));
                }
                let up = match tokens[3] {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(ParseError::InvalidFormat(format!(
                            "invalid up flag at line {}",
                            line_idx + 1
                        )));
                    }
                };

                reports.push(ChargerReport {
                    charger: ChargerId(charger),
                    interval: Interval { start, end },
                    up,
                });
            }
            Section::None => {
                return Err(ParseError::InvalidFormat(format!(
                    "unexpected content before header at line {}",
                    line_idx + 1
                )));
            }
        }
    }

    if !saw_stations || !saw_reports {
        return Err(ParseError::InvalidFormat(
            "missing required sections".into(),
        ));
    }
    if reports.is_empty() {
        return Err(ParseError::InvalidFormat(
            "no charger availability reports found".into(),
        ));
    }

    Ok((stations, reports))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChargerId, Interval, StationId};

    #[test]
    fn parse_valid_input_minimal() {
        let input = "[Stations]\n1 100\n\n[Charger Availability Reports]\n100 0 100 true\n";
        let (stations, reports) = parse_input(input).unwrap();
        assert_eq!(stations.len(), 1);
        assert_eq!(stations[0].id, StationId(1));
        assert_eq!(stations[0].chargers, vec![ChargerId(100)]);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].charger, ChargerId(100));
        assert_eq!(reports[0].interval, Interval { start: 0, end: 100 });
        assert!(reports[0].up);
    }

    #[test]
    fn parse_invalid_header() {
        let input = "[Bad]\n1 100\n\n[Charger Availability Reports]\n100 0 100 true\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn parse_missing_sections() {
        let input = "1 100\n100 0 100 true\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn parse_requires_at_least_one_report() {
        let input = "[Stations]\n1 100\n\n[Charger Availability Reports]\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn duplicate_station_id_rejected() {
        let input = "[Stations]\n1 100\n1 101\n\n[Charger Availability Reports]\n100 0 10 true\n101 0 10 false\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn charger_in_multiple_stations_rejected() {
        let input = "[Stations]\n1 100\n2 100\n\n[Charger Availability Reports]\n100 0 10 true\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn report_for_unknown_charger_rejected() {
        let input = "[Stations]\n1 100\n\n[Charger Availability Reports]\n999 0 10 true\n";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn duplicate_header_rejected() {
        let input = "[Stations]\n1 100\n[Stations]\n2 200\n\n[Charger Availability Reports]\n100 0 10 true\n";
        assert!(parse_input(input).is_err());
    }
}
