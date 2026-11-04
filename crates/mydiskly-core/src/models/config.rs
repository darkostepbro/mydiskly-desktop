use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub start_on_boot: bool,
    pub run_in_tray: bool,
    pub alert_threshold: u32,
    pub notify_mode: String,         // "500mb", "800mb", "1gb", "2gb", "custom", "never"
    pub notify_custom_value: u32,    // numeric value for custom threshold
    pub notify_custom_unit: String,  // "MB" or "GB"
    pub app_name: String,
    pub version: String,
    pub publisher: String,
    pub author: String,
    pub model: String,
    pub watermark: String,
    pub repo: String,
    pub website: String,
}

impl AppConfig {
    pub fn get_notify_threshold_bytes(&self) -> Option<u64> {
        match self.notify_mode.to_lowercase().as_str() {
            "500mb" => Some(500 * 1024 * 1024),
            "800mb" => Some(800 * 1024 * 1024),
            "1gb" => Some(1024 * 1024 * 1024),
            "2gb" => Some(2 * 1024 * 1024 * 1024),
            "custom" => {
                let val = (self.notify_custom_value.max(1)) as u64;
                if self.notify_custom_unit.eq_ignore_ascii_case("GB") {
                    Some(val * 1024 * 1024 * 1024)
                } else {
                    Some(val * 1024 * 1024)
                }
            }
            "never" => None,
            _ => Some(500 * 1024 * 1024),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "id".to_string(),
            start_on_boot: false,
            run_in_tray: true,
            alert_threshold: 15,
            notify_mode: "500mb".to_string(),
            notify_custom_value: 500,
            notify_custom_unit: "MB".to_string(),
            app_name: "MyDiskly".to_string(),
            version: "1.1.2026".to_string(),
            publisher: "Seroja Labs".to_string(),
            author: "Darkostepbro".to_string(),
            model: "Collaboration with Smart AI".to_string(),
            watermark: "by Seroja Labs".to_string(),
            repo: "https://github.com/darkostepbro/mydiskly-desktop".to_string(),
            website: "https://serojalabs.sbs/mydiskly".to_string(),
        }
    }
}
