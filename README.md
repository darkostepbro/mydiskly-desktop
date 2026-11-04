# MyDiskly (Pure Native Rust Edition) 💽
> **A Super Lightweight, Modern & Fluent Windows 11 & 10 Storage Scanner, Telemetry & Cleaner Suite**

<p align="center">
  <img src="assets/icon.png" width="128" height="128" alt="MyDiskly Native Icon" />
</p>

<p align="center">
  <a href="https://github.com/darkostepbro/mydiskly-desktop/releases"><img src="https://img.shields.io/badge/Release-v1.1.2026-0B65E6?style=for-the-badge&logo=windows11&logoColor=white" alt="Release v1.1.2026" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPL--3.0-green?style=for-the-badge" alt="GPL-3.0 License" /></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 2021" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop"><img src="https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-blue?style=for-the-badge&logo=windows" alt="Platform" /></a>
  <a href="https://github.com/darkostepbro/mydiskly-desktop/issues"><img src="https://img.shields.io/badge/Community%20Feedback-Welcome-orange?style=for-the-badge&logo=github" alt="Feedback" /></a>
</p>

---

## 📖 About MyDiskly
**MyDiskly** is a high-performance Windows desktop storage telemetry and maintenance utility built **100% in pure native Rust** (`eframe` / `egui 0.29` with native Glow Win32 rendering).

Designed for Windows 10 and Windows 11 (including debloated and custom builds such as *Ghost Spectre*, *AtlasOS*, *Tiny11*, and *ReviOS*), MyDiskly delivers **ultra-low memory consumption**, instant startup, and a sleek vertical docking layout—completely free from Python runtimes, WebView2, Chromium processes, and web-view overhead.

> **Publisher**: [Seroja Labs](https://serojalabs.sbs/mydiskly)  
> **Author**: [Darkostepbro](https://github.com/darkostepbro)  
> **Version**: `1.1.2026`  
> **Category**: Native Installer & Portable Storage Suite  
> **License**: GNU General Public License v3.0 (GPL-3.0)  

---

## 🚀 Performance Comparison: Legacy Python vs. Pure Native Rust

| Metric | Legacy Version (Python / PyInstaller) | Current Version (Pure Native Rust) | Improvement |
| :--- | :--- | :--- | :--- |
| **Language & Engine** | Python 3.11 + Tkinter / ctypes | **100% Rust 2021 (Win32 + Glow)** | Zero Runtime Dependencies |
| **Active Memory (RAM)** | ~110 MB – 180 MB | **~30 MB – 40 MB** | **~80% Memory Reduction** |
| **Idle Memory (Tray)** | ~75 MB – 90 MB | **~3 MB – 5 MB** | **~95% Memory Reduction** |
| **Cold Startup Time** | 2.5 – 4.0 seconds | **< 50 milliseconds** | **50x Faster Launch** |
| **External Dependencies** | Python DLLs, MSVCRT, WebView2 | **None (Self-Contained Executable)** | 100% Standalone |
| **OS Compatibility** | Prone to crashes on debloated Windows | **Windows 10 & 11 x64 (All Editions)** | Fully Resilient |

---

## ✨ Key Features

### 1. 🖥️ Vertical Right-Docking & Native Acrylic Dark Blur
- Seamless vertical docking along the right edge of the desktop work area (`SPI_GETWORKAREA`) at a fixed **420 px** width.
- Always-On-Top capability with minimalist **Close `[x]`** and **Minimize `[—]`** window controls.
- Native Windows Acrylic Dark Blur composition powered by `SetWindowCompositionAttribute` and `DwmSetWindowAttribute` (`DWMWCP_ROUND`).
- Instant runtime bilingual toggle **`[ID | EN]`** and dynamic permission indicators: **`[User mode]`** (dimmed) and **`[Admin mode]`** (bright).

### 2. 💽 Interactive Partitions & Real-Time Telemetry
- **Interactive Partition Cards**: Clean visual capacity bars adhering to ergonomic thresholds (Normal / Warning / Critical). Clicking any partition reveals quick actions:
  - `1. Open file`: Opens the root drive directly in Windows File Explorer.
  - `2. Properties`: Invokes the native Windows Drive Properties dialog.
- **S.M.A.R.T Drive Diagnostics**: Physical drive identification (NVMe, SSD, HDD) with bold health percentages (**Bold %**), sensor temperatures (**Bold °C**), and total power-on hours.
- **Docked Read / Write Waveform Graph**: Sub-pixel 60 FPS real-time rate graph with a 30-second rolling telemetry buffer anchored at the bottom of the Dashboard.

### 3. 🧹 Safe Smart Cleaner with Isolated Browser Cache
- **Isolated Browser Cache Scanner**: Automatically detects installed Google Chrome, Brave Browser, and Microsoft Edge profiles.
- **Guaranteed Safe**: Cleans only cache stores (*System Cache, Code Cache, GPUCache, Service Worker Cache*). **Never deletes** browsing history, bookmarks, cookies, saved passwords, or login sessions.
- **Administrative Protection Shield**: System cache items requiring elevated privileges are automatically set to read-only during User Mode, with a one-click **Run as Administrator** prompt.
- **Visual Progress & Softblur Confirmation Dialog**: Smooth progress animations during cleaning and an elegant completion modal dialog with background softblur.

### 4. ⚙️ Hardware Specifications & Audio Alert System
- **Structured Hardware Specs Table**: Displays 15 technical hardware parameters (OS Build, CPU, GPU, RAM, Motherboard, BIOS Mode, Storage, Audio, Network, Peripherals) without horizontal scrolling.
- **Proactive Cache Notification Alerts**: Configurable cache size reminders (`500MB`, `800MB`, `1GB`, `2GB`, `Custom MB/GB` with auto-rollover, `Never`).
- **Native Audio Effect**: Plays an authentic interface sound (`interface.wav`) upon completing cleanup tasks or triggering threshold notifications.
- **Focus Assist & System Notification Aware**: Automatically detects Windows Toast settings and Focus Assist state.

### 5. 📁 Self-Synchronizing Persistence Components
MyDiskly automatically maintains and updates three configuration components:
- **`mydiskly.ini`**: Stores user preferences (theme, language, startup registry, and alert thresholds).
- **`myspecs.ini`**: Formatted with the device header `[{user} - {computer}]` and 15 complete hardware parameters. Automatically refreshed on hardware upgrades or USB drive hotplug events.
- **`historydiskhealth.ini`**: Date-stamped partition health log preserving historical inspection records (`[02/nov/2026]`, etc.).

### 6. 📦 Custom Setup & Clean Uninstaller Suite (`480 × 300` px)
- Dedicated standalone installer (**`MyDiskly-Setup.exe`**) and uninstaller (**`Uninstall.exe`**).
- **3-Step Roadmap**: `1. Welcome` ➔ `2. Installing` ➔ `3. Complete` with vector checkmarks and full-rounded pill switches.
- **Full-Rounded Pill Toggles**: `Create the shortcut`, `Run as Administrator`, `Pin on Start Menu`, `Open File Location`.
- **Silent Background Execution**: Eliminates command-prompt flashes using Win32 `CREATE_NO_WINDOW` and `ShellExecuteW`.
- **Clean Installation Footprint**: Exactly 6 files deployed to `%LOCALAPPDATA%\Programs\MyDiskly`:
  1. `MyDiskly.exe`
  2. `Uninstall.exe`
  3. `mydiskly.ini`
  4. `myspecs.ini`
  5. `historydiskhealth.ini`
  6. `LICENSE.txt`
- **Permanent Clean Uninstall**: Terminates any active or elevated background instances using a 4-layer force-kill mechanism, unlocks the installation folder, and removes all files and registry keys without residue.

---

## 🏛️ Cargo Workspace Layout

```
mydiskly-desktop/
├── Cargo.toml                         # Cargo Root Workspace
├── Cargo.lock                         # Locked Dependency Manifest
├── crates/
│   └── mydiskly-core/                 # Shared Win32 Telemetry & System Library
│       ├── src/
│       │   ├── lib.rs                 # Library Entrypoint
│       │   ├── gatekeeper.rs          # Windows 10 & 11 OS Verification
│       │   ├── disk_telemetry.rs      # Partition Telemetry & 30s I/O Buffer
│       │   ├── smart_diagnostics.rs   # Physical Drive S.M.A.R.T Sensors
│       │   ├── hotplug_monitor.rs     # USB Hotplug Event Detection
│       │   ├── hardware_specs.rs      # 15-Parameter Hardware Audit Engine
│       │   ├── cleaner_engine.rs      # Multi-Category Cache & Browser Cleaner
│       │   ├── ini_manager.rs         # INI Persistence (Config, Specs, History)
│       │   └── models/                # Telemetry & Configuration Structs
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
- **Rust Toolchain**: `stable-x86_64-pc-windows-gnu` or `stable-x86_64-pc-windows-msvc` (Rust 1.75+)
- **LLVM-MinGW / GCC**: Required for Win32 linking and resource compilation (`windres`)

### Building the Binaries

#### Option A: One-Click Automated Pipeline (Recommended)
Run the automated build script from the root directory:
```powershell
.\build.ps1
```
*(Or execute `build.bat` from a standard Command Prompt)*

#### Option B: Manual Step-by-Step Compilation
```powershell
# 1. Compile the Main Desktop Application (MyDiskly.exe)
cargo build --release -p mydiskly-app

# 2. Compile the Standalone Setup Wizard (MyDiskly-Setup.exe)
cargo build --release -p mydiskly-installer

# 3. Synchronize the Standalone Uninstaller Suite
Copy-Item target/release/MyDiskly-Setup.exe target/release/Uninstall.exe -Force
```

Compiled binaries will be generated inside `target/release/`:
* `target/release/MyDiskly.exe` (Primary Desktop Application)
* `target/release/MyDiskly-Setup.exe` (Setup Wizard & Standalone Installer)
* `target/release/Uninstall.exe` (Clean Uninstaller Suite)

---

## 📄 License
This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. You are free to inspect, modify, and redistribute this software under the same open-source terms.

Copyright (C) 2026 **Seroja Labs** & **Darkostepbro**. All rights reserved.  
Official Website: [serojalabs.sbs/mydiskly](https://serojalabs.sbs/mydiskly)  
Source Code: [github.com/darkostepbro/mydiskly-desktop](https://github.com/darkostepbro/mydiskly-desktop)
