# Changelog - MyDiskly 💽

All notable changes to the **MyDiskly** desktop suite will be documented in this file.

---

## [1.1.2026] - 2026-11-04
### 🚀 MyDiskly v1.1.2026 - Porting to RUST!

#### 🌟 Architectural Breakthrough
- **100% Pure Native Rust Engine**: Fully migrated from Python / PyInstaller to pure native Rust (`eframe` / `egui 0.29` with Glow native Win32 rendering).
- **Zero External Dependencies**: Completely removed Python runtime, ctypes wrappers, Chromium, and WebView2.
- **Ultra-Low Memory Footprint**:
  - Reduced active memory usage by up to **80%** (~30–40 MB working set).
  - Reduced idle memory down to **~3–5 MB** when minimized to System Tray using native Win32 `SetProcessWorkingSetSize`.
- **Instant Cold Startup**: Launch time reduced from ~3.5 seconds down to **< 50 milliseconds**.

#### ✨ Features & Enhancements
- **Vertical Right-Docking**: Always-On-Top 420px vertical dock aligned to Windows work area (`SPI_GETWORKAREA`) with native Acrylic Dark Blur (`SetWindowCompositionAttribute` + `DwmSetWindowAttribute`).
- **Interactive Partition Explorer**: Click partition card to open file explorer or view drive properties dialog directly.
- **S.M.A.R.T Drive Diagnostics**: Physical storage detection (NVMe, SSD, HDD) with bold health percentage (`%`) and bold sensor temperature (`°C`).
- **30-Second Real-Time I/O Waveform Graph**: Docked sub-pixel 60 FPS read/write rate graph at the bottom of the Dashboard.
- **Smart Cleaner with Browser Isolation**:
  - Dedicated, isolated browser cache scanner for Google Chrome, Brave, and Microsoft Edge.
  - 100% safe: guaranteed zero impact on browsing history, cookies, bookmarks, or login sessions.
  - System cache protection with automatic `[User mode]` / `[Admin mode]` status badge and one-click elevation.
  - Softblur background completion modal popup with cleaned storage metrics.
- **Hardware Specs & Notification Alert System**:
  - Responsive 15-parameter hardware audit table without horizontal scrolling.
  - Cache threshold notification reminders (`500MB`, `800MB`, `1GB`, `2GB`, `Custom MB/GB` with auto-rollover, `Never`) with Windows Toast detection and native `interface.wav` sound effect.
- **Component Persistence**:
  - `mydiskly.ini`: Application configuration.
  - `myspecs.ini`: Hardware specifications starting with `[{user} - {computer}]`, auto-updated on hardware change or USB hotplug.
  - `historydiskhealth.ini`: Historical date-stamped partition health log.
- **Custom Setup & Uninstaller Suite (`480 × 300` px)**:
  - 3-step setup roadmap (`Welcome` ➔ `Installing` ➔ `Complete`) with full-rounded pill switches.
  - 100% silent background execution (`CREATE_NO_WINDOW` & Win32 `ShellExecuteW`).
  - Dual-mode executable acting as `MyDiskly-Setup.exe` and `Uninstall.exe`.
  - Clean deployment with exactly 6 files in `%LOCALAPPDATA%\Programs\MyDiskly`.
  - 4-layer force-termination and permanent clean uninstallation.

---

## [1.0.2026] - 2026-10-28
### Initial Release (Python Prototype)
- Initial prototype based on Python 3.11, CustomTkinter, and `disktrex.dll`.
