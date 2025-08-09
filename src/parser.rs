use crate::types::{ChargerReport, Station};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

pub fn parse_input(_input: &str) -> Result<(Vec<Station>, Vec<ChargerReport>), ParseError> {
    // Placeholder; to be implemented in Commit 3
    Err(ParseError::InvalidFormat("not implemented".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChargerId, Interval, StationId};

    #[test]
    #[ignore]
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
    #[ignore]
    fn parse_invalid_header() {
        let input = "[Bad]\n1 100\n\n[Charger Availability Reports]\n100 0 100 true\n";
        assert!(parse_input(input).is_err());
    }
}
