use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

use crate::models::{AppConfig, PartitionInfo, SmartDiskInfo, SystemSpecs};

const RUN_REG_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_REG_VALUE: &str = "MyDiskly";

#[repr(C)]
struct SystemTime {
    w_year: u16,
    w_month: u16,
    w_day_of_week: u16,
    w_day: u16,
    w_hour: u16,
    w_minute: u16,
    w_second: u16,
    w_milliseconds: u16,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetLocalTime(lpSystemTime: *mut SystemTime);
}

pub fn get_today_date_tag() -> String {
    let mut st = SystemTime {
        w_year: 2026,
        w_month: 11,
        w_day_of_week: 0,
        w_day: 2,
        w_hour: 0,
        w_minute: 0,
        w_second: 0,
        w_milliseconds: 0,
    };
    unsafe {
        GetLocalTime(&mut st);
    }
    let month_str = match st.w_month {
        1 => "jan",
        2 => "feb",
        3 => "mar",
        4 => "apr",
        5 => "may",
        6 => "jun",
        7 => "jul",
        8 => "aug",
        9 => "sep",
        10 => "oct",
        11 => "nov",
        12 => "dec",
        _ => "nov",
    };
    format!("[{:02}/{}/{}]", st.w_day, month_str, st.w_year)
}

pub fn get_app_base_dir() -> PathBuf {
    if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
        let p = Path::new(&local_app).join("Programs").join("MyDiskly");
        if p.exists() {
            return p;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.to_path_buf();
        }
    }
    PathBuf::from(".")
}

pub fn get_ini_path() -> PathBuf {
    get_app_base_dir().join("mydiskly.ini")
}

pub fn get_myspecs_ini_path() -> PathBuf {
    get_app_base_dir().join("myspecs.ini")
}

pub fn get_history_health_ini_path() -> PathBuf {
    get_app_base_dir().join("historydiskhealth.ini")
}

/// Synchronizes `myspecs.ini` starting with `{user}` & computer name.
/// Automatically detects if any hardware component changed and updates both the installed dir and current dir.
pub fn sync_myspecs_ini(specs: &SystemSpecs) -> bool {
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "USER".to_string());
    let computer = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "DESKTOP-PC".to_string());

    let content = format!(
        "[{user} - {computer}]\n\
        User = {user}\n\
        ComputerName = {computer}\n\n\
        [HardwareSpecifications]\n\
        OperatingSystem = {os}\n\
        Architecture = {arch}\n\
        Processor = {cpu}\n\
        GraphicsCard = {gpu}\n\
        InstalledRAM = {ram}\n\
        Motherboard = {mobo}\n\
        BIOSMode = {bios}\n\
        StorageDrives = {storage}\n\
        DriveFirmware = {fw}\n\
        DisplayMonitor = {mon}\n\
        AudioEndpoint = {audio}\n\
        Peripherals = {periph}\n\
        ActiveNetwork = {net}\n\
        WiFiAdapter = {wifi}\n\
        LANEthernet = {lan}\n",
        user = user,
        computer = computer,
        os = specs.os,
        arch = specs.arch,
        cpu = specs.cpu,
        gpu = specs.gpu,
        ram = specs.ram,
        mobo = specs.mobo,
        bios = specs.bios_mode,
        storage = specs.storage,
        fw = specs.drive_fw,
        mon = specs.monitor,
        audio = specs.audio,
        periph = specs.peripherals,
        net = specs.net_active,
        wifi = specs.net_wifi,
        lan = specs.net_lan,
    );

    let primary_path = get_myspecs_ini_path();
    let mut changed = true;
    if let Ok(existing) = fs::read_to_string(&primary_path) {
        if existing.trim() == content.trim() {
            changed = false;
        }
    }

    if changed {
        if let Some(parent) = primary_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&primary_path, &content);
    }

    // Also mirror to local working directory if different
    let local_path = PathBuf::from("myspecs.ini");
    if local_path != primary_path {
        let _ = fs::write(local_path, &content);
    }

    changed
}

/// Synchronizes `historydiskhealth.ini` with date-stamped disk health status blocks.
/// Preserves historical date sections (`[01/nov/2026]`, `[24/apr/2028]`, etc.) and updates today's date block.
pub fn sync_history_disk_health_ini(
    partitions: &[PartitionInfo],
    smart_disks: &[SmartDiskInfo],
) -> bool {
    let today_tag = get_today_date_tag();

    // Determine primary health percentage from S.M.A.R.T drives
    let default_wear = smart_disks
        .first()
        .map(|d| d.wear_life.as_str())
        .unwrap_or("100%");
    let second_wear = smart_disks
        .get(1)
        .map(|d| d.wear_life.as_str())
        .unwrap_or(default_wear);

    let mut today_lines = Vec::new();
    today_lines.push(today_tag.clone());

    for (idx, p) in partitions.iter().enumerate() {
        let drive_letter = p.drive.trim_end_matches('\\');
        let wear_str = if idx == 0 { default_wear } else { second_wear };
        let numeric_pct: u32 = wear_str
            .trim_end_matches('%')
            .trim()
            .parse()
            .unwrap_or(100);

        let status_lbl = if numeric_pct >= 80 && p.percent_free >= 5.0 {
            "Safe"
        } else if numeric_pct >= 50 {
            "Warning"
        } else {
            "Critical"
        };

        if idx == 0 {
            today_lines.push(format!("disk {} {} ({})", drive_letter, wear_str, status_lbl));
        } else {
            today_lines.push(format!("{} {} ({})", drive_letter, wear_str, status_lbl));
        }
    }

    let today_block = today_lines.join("\n");
    let primary_path = get_history_health_ini_path();

    let final_content = if let Ok(existing) = fs::read_to_string(&primary_path) {
        if let Some(pos) = existing.find(&today_tag) {
            // Replace today's block while keeping all historical blocks before it
            let prefix = existing[..pos].trim_end();
            if prefix.is_empty() {
                format!("history checking health :\n\n{}\n", today_block)
            } else {
                format!("{}\n\n{}\n", prefix, today_block)
            }
        } else {
            let trimmed = existing.trim();
            if trimmed.is_empty() {
                format!("history checking health :\n\n{}\n", today_block)
            } else {
                format!("{}\n\n{}\n", trimmed, today_block)
            }
        }
    } else {
        format!("history checking health :\n\n{}\n", today_block)
    };

    if let Some(parent) = primary_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let ok = fs::write(&primary_path, &final_content).is_ok();

    let local_path = PathBuf::from("historydiskhealth.ini");
    if local_path != primary_path {
        let _ = fs::write(local_path, &final_content);
    }

    ok
}

/// Checks if MyDiskly is currently registered in Windows Startup (`HKCU\...\Run`).
pub fn is_startup_registered() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run_key) = hkcu.open_subkey(RUN_REG_KEY) {
        if let Ok(val) = run_key.get_value::<String, _>(APP_REG_VALUE) {
            return !val.trim().is_empty();
        }
    }
    false
}

/// Registers or removes MyDiskly from Windows Startup Registry (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`).
pub fn sync_startup_registry(enable: bool) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((run_key, _)) = hkcu.create_subkey(RUN_REG_KEY) {
        if enable {
            if let Ok(exe_path) = std::env::current_exe() {
                let quoted = format!("\"{}\"", exe_path.to_string_lossy());
                return run_key.set_value(APP_REG_VALUE, &quoted).is_ok();
            }
        } else {
            let _ = run_key.delete_value(APP_REG_VALUE);
            return true;
        }
    }
    false
}

pub fn load_config() -> AppConfig {
    let path = get_ini_path();
    let mut config = AppConfig::default();

    if let Ok(content) = fs::read_to_string(path) {
        let mut section = "DEFAULT".to_string();
        let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                section = trimmed[1..trimmed.len() - 1].trim().to_uppercase();
                map.entry(section.clone()).or_default();
            } else if let Some((k, v)) = trimmed.split_once('=') {
                map.entry(section.clone())
                    .or_default()
                    .insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }

        if let Some(prefs) = map.get("PREFERENCES") {
            if let Some(t) = prefs.get("theme") {
                config.theme = t.clone();
            }
            if let Some(l) = prefs.get("language") {
                config.language = l.clone();
            }
            if let Some(s) = prefs.get("startonboot") {
                config.start_on_boot = s == "true" || s == "1";
            }
            if let Some(r) = prefs.get("runintray") {
                config.run_in_tray = r == "true" || r == "1";
            }
            if let Some(a) = prefs.get("alertthreshold") {
                if let Ok(val) = a.parse() {
                    config.alert_threshold = val;
                }
            }
            if let Some(nm) = prefs.get("notifymode") {
                config.notify_mode = nm.clone();
            }
            if let Some(ncv) = prefs.get("notifycustomvalue") {
                if let Ok(val) = ncv.parse() {
                    config.notify_custom_value = val;
                }
            }
            if let Some(ncu) = prefs.get("notifycustomunit") {
                let u = ncu.to_uppercase();
                if u == "GB" || u == "MB" {
                    config.notify_custom_unit = u;
                }
            }
        }
    }

    if is_startup_registered() {
        config.start_on_boot = true;
    }

    config
}

pub fn save_config(config: &AppConfig) -> bool {
    let _ = sync_startup_registry(config.start_on_boot);

    let path = get_ini_path();
    let content = format!(
        "; ====================================================================\n\
        ;  MyDiskly Configuration & Metadata\n\
        ;  Publisher: {}\n\
        ;  Author: {}\n\
        ;  Model: {}\n\
        ;  Copyright (c) 2026 {} & {}. All rights reserved.\n\
        ; ====================================================================\n\n\
        [Metadata]\n\
        ApplicationName = {}\n\
        Version = {}\n\
        ReleaseType = Installer App\n\
        Publisher = {}\n\
        Author = {}\n\
        Model = {}\n\
        Origin = Made in Indonesia\n\
        License = GPL License\n\
        Repository = {}\n\
        Website = {}\n\
        Watermark = {}\n\n\
        [Preferences]\n\
        Theme = {}\n\
        Language = {}\n\
        StartOnBoot = {}\n\
        RunInTray = {}\n\
        AlertThreshold = {}\n\
        NotifyMode = {}\n\
        NotifyCustomValue = {}\n\
        NotifyCustomUnit = {}\n",
        config.publisher,
        config.author,
        config.model,
        config.publisher,
        config.author,
        config.app_name,
        config.version,
        config.publisher,
        config.author,
        config.model,
        config.repo,
        config.website,
        config.watermark,
        config.theme,
        config.language,
        config.start_on_boot,
        config.run_in_tray,
        config.alert_threshold,
        config.notify_mode,
        config.notify_custom_value,
        config.notify_custom_unit
    );

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let ok = fs::write(&path, &content).is_ok();
    let local_path = PathBuf::from("mydiskly.ini");
    if local_path != path {
        let _ = fs::write(local_path, &content);
    }
    ok
}
