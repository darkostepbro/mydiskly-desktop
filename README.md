# MyDiskly (Pure Native Rust Edition) 💽
> **A Super Lightweight, Modern & Fluent Windows 11 & 10 Storage Scanner, Telemetry & Cleaner Suite**

<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="MyDiskly Native Icon" />
</p>

<p align="center">
  <a href="https://github.com/darkostepbro/mydiskly-desktop/releases"><img src="https://img.shields.io/badge/Release-v1.2.0--hotfix-0B65E6?style=for-the-badge&logo=windows11&logoColor=white" alt="Release v1.2.0-hotfix" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-green?style=for-the-badge" alt="GPL-3.0 License" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 2021" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop"><img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-blue?style=for-the-badge&logo=windows" alt="Platform" /></a>
  <a href="README_ID.md"><img src="https://img.shields.io/badge/Dokumentasi-Bahasa%20Indonesia-red?style=for-the-badge" alt="Indonesian Docs" /></a>
</p>

---

## 📖 About MyDiskly v1.2.0-hotfix

**MyDiskly** is a high-performance Windows desktop storage telemetry, hardware-aware shredding, and maintenance suite built **100% in pure native Rust** (`eframe` / `egui 0.29` with native Glow Win32 hardware acceleration).

Designed specifically for Windows 10 and Windows 11 (including debloated and customized builds such as *Ghost Spectre*, *AtlasOS*, *Tiny11*, and *ReviOS*), MyDiskly delivers **ultra-low memory consumption (3–5 MB idle)**, instantaneous sub-50ms startup, and an ergonomic vertical docking layout—completely independent of Python runtimes, WebView2, Electron, or Chromium overhead.

> **Publisher**: [Seroja Labs](https://serojalabs.sbs/mydiskly)  
> **Author**: [Darkostepbro](https://github.com/darkostepbro)  
> **Version**: `1.2.0-hotfix`  
> **Target**: Windows 10 & 11 x64 (Native Win32 Elevated Manifest)  
> **License**: GNU General Public License v3.0 (GPL-3.0)  

---

## 🚀 Performance Comparison: Legacy Python vs. Pure Native Rust

| Metric | Legacy Version (Python / PyInstaller) | Current Version (Pure Native Rust v1.2.0) | Improvement |
| :--- | :--- | :--- | :--- |
| **Language & Engine** | Python 3.11 + Tkinter / ctypes | **100% Rust 2021 (Win32 + Glow)** | Zero Runtime Dependencies |
| **Active Memory (RAM)** | ~110 MB – 180 MB | **~30 MB – 40 MB** | **~80% Memory Reduction** |
| **Idle Memory (Tray)** | ~75 MB – 90 MB | **~3 MB – 5 MB** | **~95% Memory Reduction** |
| **Cold Startup Time** | 2.5 – 4.0 seconds | **< 50 milliseconds** | **50x Faster Launch** |
| **External Dependencies** | Python DLLs, MSVCRT, WebView2 | **None (Self-Contained Executable)** | 100% Standalone |
| **OS Compatibility** | Prone to crashes on debloated Windows | **Windows 10 & 11 x64 (All Editions)** | Fully Resilient |
| **SSD File Sanitization** | Blind Multi-Pass Overwrites (NAND Wear) | **Hardware-Aware Zero-TBW (`FSCTL_FILE_LEVEL_TRIM`)** | **0-Byte Write Amplification** |
| **Self-Updater Safety** | In-place uncoordinated overwrite | **Defender-Safe Atomic Rename + Rollback** | **Zero Sharing Violations** |

---

## ⚡ Key Highlights of v1.2.0-hotfix

### 1. 🛡️ Hardware-Aware Zero-TBW File Deallocation (`shredder.rs`)
* **Physical Seek-Penalty Query (Cached in RAM):** Opens Win32 volume handle (`\\.\C:`) and queries `IOCTL_STORAGE_QUERY_PROPERTY` (`StorageDeviceSeekPenaltyProperty`), caching the `DriveMedium` per drive letter (`OnceLock<Mutex<HashMap<char, DriveMedium>>>`).
* **Strict Separation: File Shredder (`wipe_file_safe`) vs. Routine Cache Cleaner (`deallocate_cache_file`):**
  - **Routine Cache Cleaner (`deallocate_cache_file`):** Clears `Read-Only` attributes and executes fast metadata unlink (`std::fs::remove_file`), allowing `ntfs.sys` to dispatch batch asynchronous `DeleteNotify` (TRIM) in milliseconds with **0-Byte TBW** and zero `FlushFileBuffers` stalls across tens of thousands of small cache files.
  - **File Shredder (`wipe_file_safe`):** Locks out multi-pass overwrites on SSDs. For **MFT-Resident files ($\le 1024$ bytes — 1 NTFS MFT Record)**, scrubs the inline `$DATA` attribute with a 1-pass `0x00` write + `file.sync_all()`. For **Non-Resident files ($> 1024$ bytes)**, issues `FSCTL_FILE_LEVEL_TRIM` (with mandatory 4-byte `FILE_LEVEL_TRIM_OUTPUT` buffer) **before** `file.set_len(0)`.

### 2. 🔄 Defender-Safe Atomic Self-Updater (`updater.rs`)
* **Windows Defender (`MsMpEng.exe`) Contention Defense:** Applies a **5-retry loop with 250ms backoff** when swapping `mydiskly.exe.new` -> `mydiskly.exe`.
* **Same-Volume Atomic Rename & Fail-Safe Rollback:** Stages replacement binaries on the exact same volume and automatically restores `mydiskly.exe.bak` if replacement fails.

### 3. 🧹 5-Category Holistic Device Cache Engine (`cache_scanner.rs`)
* **5 Collapsible Windows 11 Settings-Style Categories:**
  1. **`1. Windows OS System` (Open by Default):** Windows Temp (`%TEMP%`, `C:\Windows\Temp`), Windows Update (`SoftwareDistribution\Download`), Recycle Bin, **Microsoft Office 365 / 16.0** (`OfficeFileCache`, `Wef`), **WPS Office** (`office6\cache`, `addons\cef\cache`), and Windows Explorer Thumbnail Cache (`PerformanceGuard`).
  2. **`2. Web Browser`:** **Brave**, **Google Chrome**, **Microsoft Edge**, **Mozilla Firefox**, **Opera & Opera GX**, **Vivaldi**, and **Tor Browser** (`profile.default\cache2`).
  3. **`3. Developer Tools`:** **Google Antigravity IDE**, **Visual Studio Code**, and **Claude Code CLI**.
  4. **`4. Creative Suite`:** **All Adobe CC Apps** (`Common\Media Cache Files`, `CameraRaw\Cache`, `Photoshop Temp* >1d` in `SafeToDelete`; active `.mcdb` & `Peak Files` in `PerformanceGuard`), **TikTok LIVE Studio**, **Figma Desktop**, **CapCut**, and **OBS Studio**.
  5. **`5. Gaming Core`:** **VALORANT**, **Steam Client**, **Discord**, and **DirectX / GPU Driver Shader Cache** (`NVIDIA DXCache`, `AMD DxCache`, `D3DSCache`).
* **5-Layer Disk & OS Safety Interlock (`is_safe_deletion_target`):**
  1. **Minimum Path Depth (`path.components().count() >= 4`) & Vital Kernel Blocklist (`System32`, `SysWOW64`, `WinSxS`, `Boot`, `EFI`).**
  2. **Riot Vanguard (`vgk`) Strict Anti-Cheat Blacklist.**
  3. **Office AutoRecover (`.asd`, `.xar`) & WPS Auto-Backup (`office6\backup`) Protection.**
  4. **Tor Network Guard State (`TorBrowser\Data\Tor`) & User Database Protection.**
  5. **Claude Code Session Transcript (`.claude\projects` & `.jsonl`) Protection + Win32 `FILE_ATTRIBUTE_REPARSE_POINT (0x0400)` Guard.**

---

## 📊 Version History & Evolution Table

| Version | Release Tier | Key Engineering Milestones & Features |
| :--- | :--- | :--- |
| **`v1.0.0`** | Foundation Release | Visual Disk Space Treemap Analyzer, basic `%TEMP%` cleaner with Administrator UAC manifest, native Rust `egui` UI (~3.8 MB binary). |
| **`v1.1.0`** | Stable Baseline | Added SIMD-Accelerated Duplicate File Finder, Raw Multi-Pass File Shredder (DoD 5220.22-M 3-Pass & Gutmann), 0.0% idle CPU telemetry (~4.0 MB binary). |
| **`v1.2.0-hotfix`** | **Current Release** | Retains 100% of `v1.1.0` features + **5-Category Windows 11 Accordion Holistic Cache Cleaner** (MS Office, WPS Office, Opera/GX, Vivaldi, Tor, All Adobe CC Apps, TikTok LIVE Studio, Antigravity, Claude Code, Valorant), **Hardware-Aware Zero-TBW SSD Deallocation** (`<=1024B` MFT Scrub + `>1024B` `FSCTL_FILE_LEVEL_TRIM` on Shredder; Fast Unlink on Cleaner), **5th Safety Interlock (`is_safe_deletion_target`)**, **Hybrid Timestamp Engine**, **Anti-Junction `0x0400` Guard**, **App Recommendation Feedback Bar**, **Windows 11 Settings Overview Sync**, and **Top-Right Tray Menu + Help & Privacy Terms Window**. |

---

## 🏛️ System Architecture

### 1. Hardware-Aware File Sanitization Pipeline
```
[Target Path: \\?\C:\Path\to\file.dat]
                 │
                 ▼
   5th Safety Interlock: is_safe_deletion_target(path)
   (Depth >= 4, No Kernel Paths, No 0x0400 Junction, No Vanguard/.asd/.jsonl)
                 │
                 ▼
   Strip Extended Prefix "\\?\" ➔ Extract Drive Letter 'C'
                 │
                 ▼
   Cached Volume Query: \\.\C: (IOCTL_STORAGE_QUERY_PROPERTY, SeekPenalty)
                 │
        ┌────────┴────────────────────────┐
        ▼                                 ▼
[SolidStateDrive (SSD/NVMe)]     [RotationalHdd (HDD)]
        │                                 │
        ▼                                 ▼
 Check File Length                Execute Multi-Pass DoD Overwrite
        │                         (1-Pass, 3-Pass DoD, or Gutmann)
 ┌──────┴───────────────┐                 │
 ▼                      ▼                 ▼
Len <= 1024 B        Len > 1024 B         Truncate: set_len(0)
(1 MFT Record)       (Non-Resident)       │
 │                      │                 ▼
Write 0x00 Overwrite   FSCTL_FILE_LEVEL_TRIM  fs::remove_file()
 │                      (with 4-byte buffer)
file.sync_all()         │
 │                      ▼
file.set_len(0)      file.set_len(0) (Cluster Freed)
 │                      │
 ▼                      ▼
fs::remove_file()    fs::remove_file()
```

### 2. Defender-Safe Atomic Self-Updater Pipeline
```
[Remote Binary Downloaded]
           │
           ▼
Stage on Same Volume: "mydiskly.exe.new"
           │
           ▼
Compute Streaming SHA-256 (64 KB chunks)
           │
   ┌───────┴───────┐
   ▼               ▼
[Hash Mismatch]  [Hash Verified]
   │               │
Delete .new        fs::rename("mydiskly.exe", "mydiskly.exe.bak")
Abort Update       │
                   ▼
           Attempt fs::rename(".new", "mydiskly.exe")
                   │
           ┌───────┴───────────────┐
           ▼                       ▼
       [Success]              [Error 32: Sharing Violation]
           │                  (Windows Defender MsMpEng.exe lock)
           │                       │
           │                       ▼
           │             Retry Loop (Up to 5x @ 250ms sleep)
           │                       │
           │             ┌─────────┴─────────┐
           │             ▼                   ▼
           │         [Acquired]          [Exhausted 5x]
           │             │                   │
           │             │                   ▼
           │             │         FAIL-SAFE ATOMIC ROLLBACK:
           │             │         fs::rename(".bak", ".exe")
           │             │         fs::remove_file(".new")
           │             │         Preserve system integrity!
           ▼             ▼
  App Upgrade Complete! (.bak cleaned on next startup)
```

---

## 🏛️ Cargo Workspace Layout

```
mydiskly-desktop/
├── Cargo.toml                         # Cargo Root Workspace (version = "1.2.0")
├── Cargo.lock                         # Locked Dependency Manifest
├── crates/
│   └── mydiskly-core/                 # Shared Win32 Telemetry, Cache & Shredder Library
│       ├── src/
│       │   ├── lib.rs                 # Library Entrypoint & Re-exports
│       │   ├── shredder.rs            # Hardware-Aware Zero-TBW File Deallocation
│       │   ├── cache_scanner.rs       # Holistic Device Cache Engine (2-Tier, Blacklists)
│       │   ├── updater.rs             # Defender-Safe Atomic Self-Updater
│       │   ├── gatekeeper.rs          # Windows 10 & 11 OS Verification
│       │   ├── disk_telemetry.rs      # Partition Telemetry & 30s I/O Buffer
│       │   ├── smart_diagnostics.rs   # Physical Drive S.M.A.R.T Sensors
│       │   ├── hotplug_monitor.rs     # USB Hotplug Event Detection
│       │   ├── hardware_specs.rs      # 15-Parameter Hardware Audit Engine
│       │   ├── cleaner_engine.rs      # Multi-Category Cache & Browser Cleaner
│       │   ├── ini_manager.rs         # INI Persistence (Config, Specs, History)
│       │   └── models/                # Telemetry, Cache & Configuration Structs
├── apps/
│   ├── mydiskly-app/                  # Main Desktop Application (Right-Docked)
│   │   ├── src/
│   │   │   ├── main.rs                # Window Lifecycle, Docking & Visuals
│   │   │   ├── icons.rs               # Monochrome Vector Icons
│   │   │   ├── sys_integration.rs     # Tray, Acrylic Dark Blur, Registry, Audio
│   │   │   └── views/
│   │   │       ├── dashboard.rs       # Partitions, SMART Cards & I/O Graph
│   │   │       ├── cleaner.rs         # Cleaner View, Progress & Modal Dialog
│   │   │       ├── specs.rs           # Specs Table & Settings
│   │   │       └── about.rs           # About Card, Links & Watermark Footer
│   │   └── assets/                    # Embedded Resources, Fonts & Audio
│   └── mydiskly-installer/            # Standalone Setup Wizard & Uninstaller
│       └── src/
│           └── main.rs                # 3-Step Wizard, Shortcuts & Force-Cleanup
└── assets/                            # Application Branding & Visual Assets
```

---

## 🛠️ Compilation & Build Instructions

### Prerequisites
- **Operating System**: 64-bit Windows 10 / Windows 11
- **Rust Toolchain**: `stable-x86_64-pc-windows-msvc` (Rust 1.84+)
- **Administrator Privileges**: Required to build elevated manifests and run system tests

### Building the Binaries
```powershell
# 1. Check workspace integrity
cargo check --workspace

# 2. Build production release binaries with size optimizations
cargo build --release --workspace

# 3. Synchronize the Standalone Uninstaller Suite
Copy-Item target/release/mydiskly-installer.exe target/release/Uninstall.exe -Force
```

Compiled binaries will be generated inside `target/release/`:
* `target/release/mydiskly.exe` (Primary Desktop Application)
* `target/release/mydiskly-installer.exe` (Setup Wizard & Standalone Installer)
* `target/release/Uninstall.exe` (Clean Uninstaller Suite)

---

## 📄 License
This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. You are free to inspect, modify, and redistribute this software under the same open-source terms.

Copyright (C) 2026 **Seroja Labs** & **Darkostepbro**. All rights reserved.  
Official Website: [serojalabs.sbs/mydiskly](https://serojalabs.sbs/mydiskly)  
Source Code: [github.com/darkostepbro/mydiskly-desktop](https://github.com/darkostepbro/mydiskly-desktop)  
Indonesian Documentation: [README_ID.md](README_ID.md) | [RELEASE_v1.2.0_ID.md](RELEASE_v1.2.0_ID.md)
