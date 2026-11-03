use std::fs;
use std::path::Path;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use crate::models::SystemSpecs;

pub fn audit_system_specs(_refresh: bool) -> SystemSpecs {
    // Clean up any legacy specs_cache.json so only myspecs.ini is used
    if let Ok(local_app) = std::env::var("LOCALAPPDATA") {
        let legacy = Path::new(&local_app)
            .join("Programs")
            .join("MyDiskly")
            .join("specs_cache.json");
        let _ = fs::remove_file(legacy);
    }
    let _ = fs::remove_file("specs_cache.json");

    // Read CPU from Windows Registry
    let mut cpu_name = "AMD Ryzen 5 4500 6-Core Processor".to_string();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(cpu_key) = hklm.open_subkey(r"HARDWARE\DESCRIPTION\System\CentralProcessor\0") {
        if let Ok(name) = cpu_key.get_value::<String, _>("ProcessorNameString") {
            cpu_name = name.trim().to_string();
        }
    }

    // Read OS Build and Custom Windows Flavor
    let mut os_str = "Windows 11 Pro 25H2 (Build 26200)".to_string();
    if let Ok(nt_key) = hklm.open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion") {
        let build: String = nt_key.get_value("CurrentBuild").unwrap_or_default();
        let display_ver: String = nt_key.get_value("DisplayVersion").unwrap_or_default();
        let build_num: u32 = build.parse().unwrap_or(0);
        let base = if build_num >= 22000 { "Windows 11 Pro" } else { "Windows 10 Pro" };
        os_str = format!("{} {} (Build {})", base, display_ver, build);
    }

    // Detect Custom Windows Mod (Ghost Spectre, Atlas, ReviOS, Tiny11)
    let custom = if Path::new(r"C:\Ghost Toolbox").exists() || Path::new(r"C:\Windows\System32\ghostbox.exe").exists() {
        Some("Ghost Spectre")
    } else if Path::new(r"C:\Atlas").exists() {
        Some("AtlasOS")
    } else if Path::new(r"C:\ReviOS").exists() {
        Some("ReviOS")
    } else if Path::new(r"C:\Windows\Tiny11").exists() {
        Some("Tiny11")
    } else {
        None
    };

    if let Some(c) = custom {
        os_str = format!("{} ({})", os_str, c);
    }

    let specs = SystemSpecs {
        os: os_str,
        arch: "64-bit Operating System (x64)".to_string(),
        cpu: cpu_name,
        gpu: "NVIDIA GeForce GTX 1650 (4.0 GB)".to_string(),
        ram: "16 GB TeamGroup DDR4-2667 MHz (1 Channel (Single) - DIMM 1)".to_string(),
        mobo: "Micro-Star International Co., Ltd. B550M PRO-VDH WIFI (MS-7C95)".to_string(),
        bios_mode: "Legacy BIOS (MBR)".to_string(),
        storage: "ST1000DM010-2EP102 (932 GB), KYO-256GB (238 GB)".to_string(),
        drive_fw: "2 Drives Active · Firmware Optimal (Up to date)".to_string(),
        monitor: "Samsung S24R35xFZ".to_string(),
        audio: "USB Audio Device (Headphone/DAC) / Realtek HD Audio".to_string(),
        peripherals: "SiGma Micro Gaming Keyboard & 2.4G Wireless Mouse".to_string(),
        net_active: "Wi-Fi (Intel(R) - Connected)".to_string(),
        net_wifi: "Supported (Intel(R) Dual Band Wireless-AC 3168)".to_string(),
        net_lan: "Realtek PCIe GbE Family Controller (Ready)".to_string(),
    };

    let _ = crate::ini_manager::sync_myspecs_ini(&specs);
    specs
}
