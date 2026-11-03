use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub drive: String,
    pub label: String,
    pub fstype: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub free_gb: f64,
    pub percent_used: f32,
    pub percent_free: f32,
    pub color_status: String,
    pub is_removable: bool,
    pub drive_type_desc: String,
}
