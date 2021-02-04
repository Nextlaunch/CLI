#[derive(Debug, Clone)]
pub struct Snapshot {
    pub stage_count: usize,
    pub stages: Vec<StageSnapshot>,
}

#[derive(Debug, Clone)]
pub struct StageSnapshot {
    pub stage: usize,
    pub time: f64,
    pub velocity: f64,
    pub altitude: f64,
}