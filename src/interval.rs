use crate::types::Interval;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn merge_overlapping_and_adjacent_intervals() {
        let mut v = vec![
            Interval { start: 0, end: 10 },
            Interval { start: 10, end: 20 },
            Interval { start: 5, end: 15 },
        ];
        let merged = merge_intervals(&mut v);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].start, 0);
        assert_eq!(merged[0].end, 20);
    }

    #[test]
    #[ignore]
    fn merge_keeps_disjoint_intervals() {
        let mut v = vec![
            Interval { start: 0, end: 10 },
            Interval { start: 20, end: 30 },
        ];
        let merged = merge_intervals(&mut v);
        assert_eq!(merged.len(), 2);
    }
}

pub fn merge_intervals(_intervals: &mut Vec<Interval>) -> Vec<Interval> {
    // Placeholder; to be implemented in Commit 3
    _intervals.clone()
}
