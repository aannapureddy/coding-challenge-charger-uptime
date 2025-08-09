use crate::types::Interval;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn merge_keeps_disjoint_intervals() {
        let mut v = vec![
            Interval { start: 0, end: 10 },
            Interval { start: 20, end: 30 },
        ];
        let merged = merge_intervals(&mut v);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn zero_length_and_invalid_ignored() {
        let mut v = vec![
            Interval { start: 10, end: 10 }, // zero-length
            Interval { start: 20, end: 15 }, // invalid
            Interval { start: 0, end: 1 },
        ];
        let merged = merge_intervals(&mut v);
        assert_eq!(merged, vec![Interval { start: 0, end: 1 }]);
    }
}

/// Merge a list of half-open time intervals [start, end) into a set of
/// disjoint, sorted intervals. Overlapping or adjacent intervals are merged.
/// Invalid or zero-length intervals (end <= start) are ignored.
pub fn merge_intervals(intervals: &mut Vec<Interval>) -> Vec<Interval> {
    // Filter out invalid/zero-length intervals early
    intervals.retain(|iv| iv.end > iv.start);

    // Sort by start time; deterministic order helps testing
    intervals.sort_by_key(|iv| iv.start);

    let mut merged: Vec<Interval> = Vec::with_capacity(intervals.len());
    for current in intervals.iter().copied() {
        match merged.last_mut() {
            None => merged.push(current),
            Some(last) => {
                // Half-open intervals: [a,b) and [b,c) are adjacent and mergeable
                if current.start <= last.end {
                    if current.end > last.end {
                        last.end = current.end;
                    }
                } else {
                    merged.push(current);
                }
            }
        }
    }

    merged
}
