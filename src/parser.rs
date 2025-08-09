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
    let mut stations: Vec<Station> = Vec::new();
    let mut reports: Vec<ChargerReport> = Vec::new();

    for (line_idx, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "[Stations]" {
            section = Section::Stations;
            continue;
        }
        if line == "[Charger Availability Reports]" {
            section = Section::Reports;
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
                let mut chargers: Vec<ChargerId> = Vec::new();
                for token in parts {
                    let cid: u32 = token.parse().map_err(|_| {
                        ParseError::InvalidFormat(format!(
                            "invalid charger id at line {}",
                            line_idx + 1
                        ))
                    })?;
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

    if matches!(section, Section::None) {
        return Err(ParseError::InvalidFormat(
            "missing required sections".into(),
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
}
