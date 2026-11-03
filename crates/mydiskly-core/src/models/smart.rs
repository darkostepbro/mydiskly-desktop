use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartDiskInfo {
    pub id: String,
    pub name: String,
    pub media_type: String,
    pub bus_type: String,
    pub size_gb: f64,
    pub health_status: String,
    pub temperature: String,
    pub wear_life: String,
    pub power_hours: String,
}
