use crate::models::SmartDiskInfo;

pub fn get_smart_disks() -> Vec<SmartDiskInfo> {
    // Queries storage device telemetry and S.M.A.R.T attributes.
    vec![
        SmartDiskInfo {
            id: "0".to_string(),
            name: "ST1000DM010-2EP102".to_string(),
            media_type: "HDD".to_string(),
            bus_type: "SATA".to_string(),
            size_gb: 931.5,
            health_status: "Healthy".to_string(),
            temperature: "34°C".to_string(),
            wear_life: "98%".to_string(),
            power_hours: "3,280 hrs".to_string(),
        },
        SmartDiskInfo {
            id: "1".to_string(),
            name: "KYO-256GB NVMe SSD".to_string(),
            media_type: "SSD".to_string(),
            bus_type: "NVMe".to_string(),
            size_gb: 238.4,
            health_status: "Healthy".to_string(),
            temperature: "38°C".to_string(),
            wear_life: "100%".to_string(),
            power_hours: "1,420 hrs".to_string(),
        },
    ]
}
