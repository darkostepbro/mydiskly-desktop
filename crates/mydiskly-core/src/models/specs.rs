use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSpecs {
    pub os: String,
    pub arch: String,
    pub cpu: String,
    pub gpu: String,
    pub ram: String,
    pub mobo: String,
    pub bios_mode: String,
    pub storage: String,
    pub drive_fw: String,
    pub monitor: String,
    pub audio: String,
    pub peripherals: String,
    pub net_active: String,
    pub net_wifi: String,
    pub net_lan: String,
}
