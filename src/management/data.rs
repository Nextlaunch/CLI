use std::time::Instant;

pub mod telemetry;
pub mod launches;

#[derive(Debug, Clone)]
pub struct RenderFrame {
    pub view: usize,
    pub launch_refresh: Instant,
    pub launch: Option<launches::Launch>,
    pub telemetry: Option<telemetry::Snapshot>,
    pub error: Option<(String, bool)>,
}