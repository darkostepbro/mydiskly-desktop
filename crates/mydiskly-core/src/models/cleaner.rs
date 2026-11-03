use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerCategory {
    pub id: String,
    pub name_id: String,
    pub name_en: String,
    pub desc_id: String,
    pub desc_en: String,
    pub size_bytes: u64,
    pub size_str: String,
    pub requires_admin: bool,
    pub is_safe: bool,
    pub selected: bool,
    pub is_browser: bool,
    pub is_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
    pub success: bool,
    pub freed_bytes: u64,
    pub freed_str: String,
    pub items_cleaned: u32,
    pub message: String,
}
