use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoPoint {
    pub read_mb: f64,
    pub write_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoRateSnapshot {
    pub read_mb: f64,
    pub write_mb: f64,
    pub peak_read_mb: f64,
    pub peak_write_mb: f64,
    pub history: Vec<IoPoint>,
}
