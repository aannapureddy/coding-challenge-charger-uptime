#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StationId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChargerId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone)]
pub struct ChargerReport {
    pub charger: ChargerId,
    pub interval: Interval,
    pub up: bool,
}

#[derive(Debug, Clone)]
pub struct Station {
    pub id: StationId,
    pub chargers: Vec<ChargerId>,
}
