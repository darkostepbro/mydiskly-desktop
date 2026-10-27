"""
MyDiskly v1.0.2026 - Pure Native Windows 11 Desktop Application
Author: Darkostepbro
Repository: https://github.com/darkostepbro/mydiskly-desktop
Owner: Darkostepbro
- Powered by disktrex.dll compiled engine
- True 30% Rounded Window with Win32 Region Clipping & Zero Black Corners
- Realtime I/O Read/Write Speed (Homepage only)
- Asynchronous Registry & System Tray Loading Feedback
- Resilient Smart Cleaner with Admin Elevation & Recycle Bin Integration
- Single Instance Mutex Enforcement
- Official Fluent 3D Icon Branding with Pure Alpha Transparency
"""

import os
import sys
import time
import ctypes
from ctypes import wintypes
import threading
import webbrowser

# ─── EXPLICIT TOP-LEVEL IMPORTS FOR PYINSTALLER TREE BUNDLER ──────────
import configparser
import json
import subprocess
import winreg
from datetime import datetime

from PIL import Image
import pystray
import customtkinter as ctk

# ─── SINGLE INSTANCE MUTEX (PREVENTS MULTIPLE INSTANCES) ──────────────
MUTEX_NAME = "Global\\MyDiskly_SingleInstance_Mutex_Darkostepbro"
app_mutex = ctypes.windll.kernel32.CreateMutexW(None, False, MUTEX_NAME)
if ctypes.windll.kernel32.GetLastError() == 183:  # ERROR_ALREADY_EXISTS
    hwnd = ctypes.windll.user32.FindWindowW(None, "MyDiskly v1.0.2026")
    if hwnd:
        ctypes.windll.user32.ShowWindow(hwnd, 9)  # SW_RESTORE
        ctypes.windll.user32.SetForegroundWindow(hwnd)
    sys.exit(0)

# ─── RESOLVE APP DIRECTORY & LOAD disktrex.dll ────────────────────────
def get_root_dir():
    if getattr(sys, 'frozen', False):
        return os.path.dirname(sys.executable)
    d = os.path.dirname(os.path.abspath(__file__))
    candidates = [
        d,
        os.path.join(d, "mydiskly-v1"),
        os.path.join(os.path.dirname(d), "mydiskly-v1"),
    ]
    for c in candidates:
        if os.path.exists(os.path.join(c, "disktrex.dll")):
            return c
    return d

ROOT_DIR = get_root_dir()
dll_candidate = os.path.join(ROOT_DIR, "disktrex.dll")
if os.path.exists(dll_candidate):
    sys.path.insert(0, os.path.abspath(dll_candidate))

# Add src to sys.path for direct repository dev execution
SRC_DIR = os.path.dirname(os.path.abspath(__file__))
if SRC_DIR not in sys.path:
    sys.path.insert(0, SRC_DIR)

try:
    try:
        import disktrex
    except ImportError:
        import disktrex_engine as disktrex
    DiskApi = disktrex.DiskApi
    APP_NAME = disktrex.APP_NAME
    APP_VERSION = disktrex.APP_VERSION
    APP_PUBLISHER = getattr(disktrex, "APP_PUBLISHER", "Seroja Labs")
    APP_AUTHOR = disktrex.APP_AUTHOR
    APP_MODEL = getattr(disktrex, "APP_MODEL", "Gemini 3.8 & Claude 4.6")
    APP_REPO = disktrex.APP_REPO
    APP_WATERMARK = disktrex.APP_WATERMARK
    APP_ORIGIN = disktrex.APP_ORIGIN
    APP_LICENSE = disktrex.APP_LICENSE
    is_admin = disktrex.is_admin
    relaunch_as_admin = disktrex.relaunch_as_admin
except Exception as e:
    raise RuntimeError(f"Could not load disktrex engine: {e}")

# ─── WIN32 HIGH-DPI AWARENESS V2 ──────────────────────────────────────
try:
    ctypes.windll.shcore.SetProcessDpiAwareness(2)
except Exception:
    try:
        ctypes.windll.user32.SetProcessDPIAware()
    except Exception:
        pass

WINDOW_WIDTH = 410
WINDOW_HEIGHT = 580
TRANSPARENT_KEY = "#000001"

FONT_FAMILY = "Segoe UI Variable Text"
FONT_BOLD = (FONT_FAMILY, 12, "bold")
FONT_TITLE = (FONT_FAMILY, 15, "bold")
FONT_SUBTITLE = (FONT_FAMILY, 11)
FONT_CAPTION = (FONT_FAMILY, 10)

api = DiskApi()
app_instance = None
tray_icon = None

def get_flyout_position():
    rect = wintypes.RECT()
    ctypes.windll.user32.SystemParametersInfoW(0x0030, 0, ctypes.byref(rect), 0)
    x = rect.right - WINDOW_WIDTH - 14
    y = rect.bottom - WINDOW_HEIGHT - 14
    return max(10, x), max(10, y)

def load_brand_icon():
    base = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.path.dirname(base)
    candidates = [
        os.path.join(ROOT_DIR, "assets", "icon.png"),
        os.path.join(ROOT_DIR, "app_icon.png"),
        os.path.join(repo_root, "assets", "icon.png"),
        os.path.join(repo_root, "assets", "app_icon.png"),
        os.path.join(base, "assets", "icon.png"),
    ]
    for p in candidates:
        if os.path.exists(p):
            try:
                return Image.open(p).convert("RGBA")
            except Exception:
                pass
    return Image.new("RGBA", (64, 64), (0, 156, 255, 255))


# ─── SPLASH INITIALIZATION SCREEN (CTKTOPLEVEL TO SHARE TCL INTERP) ───
class SplashScreen(ctk.CTkToplevel):
    def __init__(self, master):
        super().__init__(master)
        self.overrideredirect(True)
        self.attributes("-topmost", True)
        self.configure(fg_color=TRANSPARENT_KEY)
        try:
            self.wm_attributes("-transparentcolor", TRANSPARENT_KEY)
        except Exception:
            pass

        w, h = 330, 210
        sw = self.winfo_screenwidth()
        sh = self.winfo_screenheight()
        x = (sw - w) // 2
        y = (sh - h) // 2
        self.geometry(f"{w}x{h}+{x}+{y}")

        self.box = ctk.CTkFrame(self, corner_radius=20, fg_color="#141416", border_width=1, border_color="#27272A")
        self.box.pack(fill="both", expand=True, padx=2, pady=2)

        raw_icon = load_brand_icon()
        self.icon_ctk = ctk.CTkImage(light_image=raw_icon, dark_image=raw_icon, size=(38, 38))

        self.lbl_logo = ctk.CTkLabel(
            self.box, text="  MyDiskly", image=self.icon_ctk, compound="left",
            font=(FONT_FAMILY, 18, "bold"), text_color="white", fg_color="transparent"
        )
        self.lbl_logo.pack(pady=(26, 2))

        # REMOVED DARKOSTEPBRO NAME ON SPLASH AS REQUESTED
        self.lbl_sub = ctk.CTkLabel(self.box, text=f"v{APP_VERSION} · Portable Edition", font=(FONT_FAMILY, 10), text_color="#A1A1AA")
        self.lbl_sub.pack(pady=(0, 18))

        self.progress = ctk.CTkProgressBar(self.box, width=240, height=5, corner_radius=3, progress_color="#0098FF")
        self.progress.pack(pady=(0, 8))
        self.progress.set(0.1)

        self.lbl_status = ctk.CTkLabel(self.box, text="Memuat modul disktrex.dll...", font=(FONT_FAMILY, 10), text_color="#71717A")
        self.lbl_status.pack()

        self.update_idletasks()
        self._apply_round_clip(w, h, 20)

    def _apply_round_clip(self, w, h, r):
        hwnd = self.winfo_id()
        p_hwnd = ctypes.windll.user32.GetParent(hwnd)
        for target in [hwnd, p_hwnd]:
            if target:
                hrgn = ctypes.windll.gdi32.CreateRoundRectRgn(0, 0, w, h, r, r)
                ctypes.windll.user32.SetWindowRgn(target, hrgn, True)

    def update_step(self, val, text):
        self.progress.set(val)
        self.lbl_status.configure(text=text)
        self.update()


# ─── MAIN APPLICATION CLASS (SINGLE TK ROOT) ─────────────────────────
class MyDisklyApp(ctk.CTk):
    def __init__(self):
        super().__init__()

        self.title(f"{APP_NAME} v{APP_VERSION}")
        self.overrideredirect(True)
        self.attributes("-topmost", True)

        self.configure(fg_color=TRANSPARENT_KEY)
        try:
            self.wm_attributes("-transparentcolor", TRANSPARENT_KEY)
        except Exception:
            pass

        self.config = api.get_config()
        self.current_theme = self.config.get("theme", "dark")
        self.is_visible = False

        ctk.set_appearance_mode(self.current_theme)
        ctk.set_default_color_theme("blue")

        self.part_cards = {}
        self.cleaner_cards = {}
        self.cleaner_vars = {}
        self.cleaner_targets_data = {}
        self.current_view = "dashboard"

        self._setup_ui()

        x, y = get_flyout_position()
        self.geometry(f"{WINDOW_WIDTH}x{WINDOW_HEIGHT}+{x}+{y}")
        self.update_idletasks()
        self._apply_window_rounding()

        self.fast_update_dashboard()
        self._start_refresh_loop()

        self.withdraw()

    def _apply_window_rounding(self):
        hwnd = self.winfo_id()
        parent_hwnd = ctypes.windll.user32.GetParent(hwnd)
        radius = 26
        for target_hwnd in [hwnd, parent_hwnd]:
            if target_hwnd:
                hrgn = ctypes.windll.gdi32.CreateRoundRectRgn(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, radius, radius)
                ctypes.windll.user32.SetWindowRgn(target_hwnd, hrgn, True)

    def _setup_ui(self):
        self.main_container = ctk.CTkFrame(
            self,
            corner_radius=24,
            fg_color=("white", "#141416"),
            border_width=1,
            border_color=("#E4E4E7", "#242428")
        )
        self.main_container.pack(fill="both", expand=True, padx=2, pady=2)
        self.main_container.grid_rowconfigure(1, weight=1)
        self.main_container.grid_columnconfigure(0, weight=1)

        # ─── MINIMALIST HEADER NAVBAR ─────────────────────────
        self.header = ctk.CTkFrame(self.main_container, height=44, corner_radius=0, fg_color="transparent")
        self.header.grid(row=0, column=0, sticky="ew", padx=16, pady=(12, 4))
        self.header.grid_columnconfigure(1, weight=1)

        raw_icon = load_brand_icon()
        self.header_icon = ctk.CTkImage(light_image=raw_icon, dark_image=raw_icon, size=(22, 22))

        self.logo_lbl = ctk.CTkLabel(
            self.header,
            text=f"  {APP_NAME}",
            image=self.header_icon,
            compound="left",
            font=(FONT_FAMILY, 14, "bold"),
            fg_color="transparent"
        )
        self.logo_lbl.grid(row=0, column=0, sticky="w")

        self.ver_lbl = ctk.CTkLabel(
            self.header,
            text=f"v{APP_VERSION}",
            font=(FONT_FAMILY, 9),
            text_color="gray",
        )
        self.ver_lbl.grid(row=0, column=1, sticky="w", padx=6)

        # Navbar Action Icons (📊 SMART, 🧹 Cleaner, ⚙️ Settings, ✕ Close)
        self.btn_smart_view = ctk.CTkButton(
            self.header, text="📊", width=30, height=28, corner_radius=8,
            fg_color=("gray90", "#202024"), hover_color=("gray80", "#2C2C32"),
            command=self.show_smart_view
        )
        self.btn_smart_view.grid(row=0, column=2, padx=3)

        self.btn_clean_view = ctk.CTkButton(
            self.header, text="🧹", width=30, height=28, corner_radius=8,
            fg_color=("gray90", "#202024"), hover_color=("gray80", "#2C2C32"),
            command=self.show_cleaner
        )
        self.btn_clean_view.grid(row=0, column=3, padx=3)

        self.btn_settings_view = ctk.CTkButton(
            self.header, text="⚙️", width=30, height=28, corner_radius=8,
            fg_color=("gray90", "#202024"), hover_color=("gray80", "#2C2C32"),
            command=self.show_settings
        )
        self.btn_settings_view.grid(row=0, column=4, padx=3)

        self.btn_close = ctk.CTkButton(
            self.header, text="✕", width=28, height=28, corner_radius=8,
            fg_color="transparent", hover_color=("gray85", "#3A1A1A"),
            text_color=("gray40", "gray70"),
            command=self.on_close_clicked
        )
        self.btn_close.grid(row=0, column=5, padx=(2, 0))

        # ─── CONTENT PAGES CONTAINER ──────────────────────────
        self.content_frame = ctk.CTkFrame(self.main_container, fg_color="transparent")
        self.content_frame.grid(row=1, column=0, sticky="nsew", padx=14, pady=(2, 10))
        self.content_frame.grid_rowconfigure(0, weight=1)
        self.content_frame.grid_columnconfigure(0, weight=1)

        self.view_dashboard = ctk.CTkFrame(self.content_frame, fg_color="transparent")
        self.view_smart = ctk.CTkFrame(self.content_frame, fg_color="transparent")
        self.view_cleaner = ctk.CTkFrame(self.content_frame, fg_color="transparent")
        self.view_settings = ctk.CTkFrame(self.content_frame, fg_color="transparent")

        self._build_dashboard_layout()
        self._build_smart_layout()
        self._build_cleaner_layout()
        self._build_settings_layout()

        self.show_dashboard()

    # ─── VIEW 1: DASHBOARD (HOMEPAGE) ─────────────────────────────────
    def _build_dashboard_layout(self):
        self.scroll_parts = ctk.CTkScrollableFrame(self.view_dashboard, fg_color="transparent")
        self.scroll_parts.pack(fill="both", expand=True, pady=(0, 4))

        self.io_footer = ctk.CTkFrame(
            self.view_dashboard,
            height=32,
            corner_radius=8,
            fg_color=("gray92", "#18181A"),
            border_width=1,
            border_color=("#E4E4E7", "#27272A"),
        )
        self.io_footer.pack(fill="x", side="bottom", padx=2, pady=(4, 0))

        self.lbl_io_read = ctk.CTkLabel(
            self.io_footer,
            text="📥 Read: 0.0 MB/s",
            font=(FONT_FAMILY, 10, "bold"),
            text_color=("#2563EB", "#60A5FA"),
        )
        self.lbl_io_read.pack(side="left", padx=12, pady=4)

        self.lbl_io_write = ctk.CTkLabel(
            self.io_footer,
            text="📤 Write: 0.0 MB/s",
            font=(FONT_FAMILY, 10, "bold"),
            text_color=("#16A34A", "#4ADE80"),
        )
        self.lbl_io_write.pack(side="right", padx=12, pady=4)

    # ─── VIEW 2: S.M.A.R.T SENSOR PAGE ───────────────────────────────
    def _build_smart_layout(self):
        top_row = ctk.CTkFrame(self.view_smart, fg_color="transparent")
        top_row.pack(fill="x", pady=(2, 8))
        btn_back = ctk.CTkButton(top_row, text="← Back", width=55, height=26, font=(FONT_FAMILY, 10), command=self.show_dashboard)
        btn_back.pack(side="left")
        title_lbl = ctk.CTkLabel(top_row, text="📊  S.M.A.R.T Hardware Diagnostics", font=(FONT_FAMILY, 12, "bold"))
        title_lbl.pack(side="left", padx=8)

        self.smart_container = ctk.CTkScrollableFrame(self.view_smart, fg_color="transparent")
        self.smart_container.pack(fill="both", expand=True)

    def populate_smart_view(self):
        for w in self.smart_container.winfo_children():
            w.destroy()

        devices = api.get_smart_data()
        for item in devices:
            card = ctk.CTkFrame(self.smart_container, fg_color=("gray95", "#1C1C1F"), corner_radius=12, border_width=1, border_color=("#E5E5EA", "#2A2A30"))
            card.pack(fill="x", pady=4)

            h_row = ctk.CTkFrame(card, fg_color="transparent")
            h_row.pack(fill="x", padx=10, pady=(8, 2))
            name_lbl = ctk.CTkLabel(h_row, text=item["name"], font=(FONT_FAMILY, 12, "bold"))
            name_lbl.pack(side="left")
            type_lbl = ctk.CTkLabel(h_row, text=f"[{item['media_type']}]", font=(FONT_FAMILY, 10), text_color="gray")
            type_lbl.pack(side="right")

            m_frame = ctk.CTkFrame(card, fg_color="transparent")
            m_frame.pack(fill="x", padx=10, pady=(4, 8))
            m_frame.grid_columnconfigure(0, weight=1)
            m_frame.grid_columnconfigure(1, weight=1)

            t_lbl = ctk.CTkLabel(m_frame, text=f"Temperature: {item['temperature']}", font=(FONT_FAMILY, 11))
            t_lbl.grid(row=0, column=0, sticky="w")

            h_lbl = ctk.CTkLabel(m_frame, text=item["wear_life"], font=(FONT_FAMILY, 11))
            h_lbl.grid(row=0, column=1, sticky="w")

            status_row = ctk.CTkLabel(m_frame, text=f"Status: {item['health_status']} · Powered: {item['power_hours']}", font=(FONT_FAMILY, 10), text_color="gray")
            status_row.grid(row=1, column=0, columnspan=2, sticky="w", pady=(4, 0))

    # ─── VIEW 3: DISK CLEANER (SOLID IN-PLACE ARCHITECTURE) ───────────
    def _build_cleaner_layout(self):
        top_row = ctk.CTkFrame(self.view_cleaner, fg_color="transparent")
        top_row.pack(fill="x", pady=(2, 4))
        btn_back = ctk.CTkButton(top_row, text="← Back", width=55, height=26, font=(FONT_FAMILY, 10), command=self.show_dashboard)
        btn_back.pack(side="left")
        title_lbl = ctk.CTkLabel(top_row, text="🧹  Smart Disk Cleaner", font=(FONT_FAMILY, 12, "bold"))
        title_lbl.pack(side="left", padx=8)

        self.admin_bar = ctk.CTkFrame(self.view_cleaner, fg_color=("gray90", "#1C1C1F"), corner_radius=8)
        self.admin_bar.pack(fill="x", pady=(2, 4))

        self.lbl_admin_badge = ctk.CTkLabel(self.admin_bar, text="", font=(FONT_FAMILY, 10, "bold"))
        self.lbl_admin_badge.pack(side="left", padx=8, pady=4)

        self.btn_elevate = ctk.CTkButton(
            self.admin_bar, text="⚡ Restart as Admin", width=120, height=22,
            font=(FONT_FAMILY, 9, "bold"), fg_color="#EAB308", text_color="#18181B",
            hover_color="#CA8A04", command=self._on_restart_admin
        )

        sel_bar = ctk.CTkFrame(self.view_cleaner, fg_color="transparent")
        sel_bar.pack(fill="x", padx=4, pady=(2, 4))

        self.var_select_all = ctk.BooleanVar(value=True)
        self.chk_select_all = ctk.CTkCheckBox(
            sel_bar, text="Pilih Semua / Deselect All", variable=self.var_select_all,
            font=(FONT_FAMILY, 10, "bold"), command=self._toggle_select_all
        )
        self.chk_select_all.pack(side="left")

        self.cleaner_container = ctk.CTkFrame(self.view_cleaner, fg_color="transparent")
        self.cleaner_container.pack(fill="both", expand=True, pady=2)

        target_specs = [
            ("recycle_bin", "Windows Recycle Bin", "0 B", False),
            ("user_temp", "User Temp Files", "0 B", False),
            ("shader_cache", "DirectX Shader Cache", "0 B", False),
            ("crash_dumps", "Crash Error Dumps", "0 B", False),
            ("windows_temp", "Windows System Temp", "0 B", True),
            ("windows_update", "Windows Update Cache", "0 B", True),
        ]

        for key, name, default_sz, need_adm in target_specs:
            card = ctk.CTkFrame(
                self.cleaner_container,
                corner_radius=8,
                fg_color=("gray95", "#18181B"),
                border_width=1,
                border_color=("#E5E5EA", "#27272A"),
            )
            card.pack(fill="x", pady=2, padx=2)

            var = ctk.BooleanVar(value=not need_adm)
            self.cleaner_vars[key] = var

            chk = ctk.CTkCheckBox(
                card,
                text=name,
                variable=var,
                font=(FONT_FAMILY, 11),
                command=self._on_item_check_changed,
            )
            chk.pack(side="left", padx=8, pady=7)

            lbl_sz = ctk.CTkLabel(
                card,
                text=default_sz,
                font=(FONT_FAMILY, 11, "bold"),
                text_color="#9CA3AF"
            )
            lbl_sz.pack(side="right", padx=10)

            self.cleaner_cards[key] = {
                "frame": card,
                "chk": chk,
                "var": var,
                "lbl": lbl_sz,
                "name": name,
                "needs_admin": need_adm,
            }

        bot_frame = ctk.CTkFrame(self.view_cleaner, fg_color="transparent")
        bot_frame.pack(fill="x", side="bottom", pady=(4, 2))

        self.lbl_clean_total = ctk.CTkLabel(bot_frame, text="Reclaimable: Scanning...", font=(FONT_FAMILY, 11, "bold"))
        self.lbl_clean_total.pack(side="left", padx=4)

        self.btn_execute_clean = ctk.CTkButton(
            bot_frame, text="Clean Selected", fg_color="#238636",
            hover_color="#2ea043", height=28, font=(FONT_FAMILY, 11, "bold"),
            command=self._execute_clean
        )
        self.btn_execute_clean.pack(side="right")

    def _toggle_select_all(self):
        new_state = self.var_select_all.get()
        user_is_elevated = is_admin()
        for key, item in self.cleaner_cards.items():
            if not item["needs_admin"] or user_is_elevated:
                item["var"].set(new_state)
        self._update_clean_button_state()

    def _on_item_check_changed(self):
        self._update_clean_button_state()

    def _update_clean_button_state(self):
        selected_count = sum(1 for v in self.cleaner_vars.values() if v.get())
        selected_bytes = 0
        for key, item in self.cleaner_targets_data.items():
            if self.cleaner_vars.get(key, ctk.BooleanVar()).get():
                selected_bytes += item.get("size", 0)

        sz_str = api._fmt_size(selected_bytes)
        if selected_count > 0 and selected_bytes > 0:
            self.btn_execute_clean.configure(
                state="normal",
                text=f"Clean Selected ({sz_str})",
                fg_color="#238636"
            )
        elif selected_count > 0:
            self.btn_execute_clean.configure(
                state="normal",
                text=f"Clean Selected ({selected_count} items)",
                fg_color="#238636"
            )
        else:
            self.btn_execute_clean.configure(
                state="disabled",
                text="Pilih Item untuk Dibersihkan",
                fg_color="gray40"
            )

    def _on_restart_admin(self):
        relaunch_as_admin()
        self.quit_app()

    def _populate_cleaner_async(self):
        user_is_elevated = is_admin()
        if user_is_elevated:
            self.lbl_admin_badge.configure(text="🛡️ Administrator: Akses Penuh Sistem Aktif", text_color=("#16A34A", "#4ADE80"))
            self.btn_elevate.pack_forget()
        else:
            self.lbl_admin_badge.configure(text="🛡️ Mode Standar (Target OS butuh hak Admin)", text_color="#A1A1AA")
            self.btn_elevate.pack(side="right", padx=6, pady=3)

        self.lbl_clean_total.configure(text="Reclaimable: Scanning...")

        def worker():
            try:
                scan = api.scan_cache()
                self.after(0, self._on_cleaner_scanned, scan)
            except Exception:
                pass

        threading.Thread(target=worker, daemon=True).start()

    def _on_cleaner_scanned(self, scan):
        self.cleaner_targets_data = scan["targets"]
        user_is_elevated = scan.get("is_admin", is_admin())
        self.lbl_clean_total.configure(text=f"Reclaimable: {scan['total_fmt']}")

        for key, item in self.cleaner_targets_data.items():
            if key in self.cleaner_cards:
                c = self.cleaner_cards[key]
                can_clean = item.get("can_clean", False)
                needs_admin = item.get("needs_admin", False)

                c["lbl"].configure(text=item["size_fmt"])

                label_title = c["name"]
                if needs_admin and not can_clean:
                    label_title += " [Perlu Admin]"
                    c["chk"].configure(text=label_title, state="disabled")
                    c["var"].set(False)
                else:
                    c["chk"].configure(text=label_title, state="normal")
                    c["var"].set(True)

        self._update_clean_button_state()

    def _execute_clean(self):
        selected_keys = [k for k, v in self.cleaner_vars.items() if v.get()]
        if not selected_keys:
            return
        self.btn_execute_clean.configure(text="Cleaning in progress...", state="disabled")

        def worker():
            try:
                res = api.clean_cache(selected_keys)
                cleaned_fmt = res["total_cleaned_fmt"]
                self.after(0, self._on_clean_finished, cleaned_fmt)
            except Exception:
                pass

        threading.Thread(target=worker, daemon=True).start()

    def _on_clean_finished(self, cleaned_fmt):
        self.lbl_clean_total.configure(text=f"✅ Cleaned {cleaned_fmt}!")
        self.btn_execute_clean.configure(text="Clean Selected", state="normal")
        self.after(2000, self._populate_cleaner_async)

    # ─── VIEW 4: SETTINGS (ASYNC FEEDBACK & APP INFO CARD) ────────────
    def _build_settings_layout(self):
        top_row = ctk.CTkFrame(self.view_settings, fg_color="transparent")
        top_row.pack(fill="x", pady=(2, 8))
        btn_back = ctk.CTkButton(top_row, text="← Back", width=55, height=26, font=(FONT_FAMILY, 10), command=self.show_dashboard)
        btn_back.pack(side="left")
        title_lbl = ctk.CTkLabel(top_row, text="⚙️  Settings", font=(FONT_FAMILY, 12, "bold"))
        title_lbl.pack(side="left", padx=8)

        # 1. Startup on [checkbox]
        row_startup = ctk.CTkFrame(self.view_settings, fg_color="transparent")
        row_startup.pack(fill="x", padx=6, pady=4)

        self.var_startup = ctk.BooleanVar(value=self.config.get("start_on_boot", False))
        self.chk_startup = ctk.CTkCheckBox(
            row_startup, text="1. Run on Windows Startup", variable=self.var_startup,
            font=(FONT_FAMILY, 11), command=self._on_startup_toggle
        )
        self.chk_startup.pack(side="left")

        self.lbl_startup_status = ctk.CTkLabel(row_startup, text="", font=(FONT_FAMILY, 10))
        self.lbl_startup_status.pack(side="right", padx=6)

        # 2. Run tray icon [checkbox]
        row_tray = ctk.CTkFrame(self.view_settings, fg_color="transparent")
        row_tray.pack(fill="x", padx=6, pady=4)

        self.var_tray = ctk.BooleanVar(value=self.config.get("run_in_tray", True))
        self.chk_tray = ctk.CTkCheckBox(
            row_tray, text="2. Minimize to System Tray", variable=self.var_tray,
            font=(FONT_FAMILY, 11), command=self._on_tray_toggle
        )
        self.chk_tray.pack(side="left")

        self.lbl_tray_status = ctk.CTkLabel(row_tray, text="", font=(FONT_FAMILY, 10))
        self.lbl_tray_status.pack(side="right", padx=6)

        # ─── APP INFORMATION CARD (EXACT USER SPECIFICATION) ───────────
        info_card = ctk.CTkFrame(
            self.view_settings,
            fg_color=("gray95", "#18181B"),
            corner_radius=12,
            border_width=1,
            border_color=("#E2E2E6", "#26262A")
        )
        info_card.pack(fill="x", padx=6, pady=(10, 2))

        app_title = ctk.CTkLabel(info_card, text=APP_NAME, font=(FONT_FAMILY, 14, "bold"), text_color=("#18181B", "#FFFFFF"))
        app_title.pack(anchor="w", padx=12, pady=(10, 1))

        ver_txt = ctk.CTkLabel(
            info_card,
            text=f"{APP_VERSION}\nPortable Apps\nby {APP_PUBLISHER}\n{APP_ORIGIN}",
            font=(FONT_FAMILY, 10), justify="left", text_color="gray"
        )
        ver_txt.pack(anchor="w", padx=12, pady=(0, 6))

        lic_txt = ctk.CTkLabel(
            info_card,
            text=f"{APP_LICENSE} | Check github for open source",
            font=(FONT_FAMILY, 10, "italic"),
            text_color=("#4B5563", "#9CA3AF")
        )
        lic_txt.pack(anchor="w", padx=12, pady=(0, 2))

        feedback_btn = ctk.CTkButton(
            info_card,
            text="Share your feedback now [github.com/darkostepbro/mydiskly-desktop]",
            font=(FONT_FAMILY, 9),
            text_color="#0098FF",
            fg_color="transparent",
            hover_color=("gray85", "#27272A"),
            height=20,
            anchor="w",
            command=lambda: webbrowser.open("https://github.com/darkostepbro/mydiskly-desktop")
        )
        feedback_btn.pack(anchor="w", padx=8, pady=(0, 8))

    # ─── ASYNC LOADING HANDLERS FOR SETTINGS ──────────────────────────
    def _on_startup_toggle(self):
        val = self.var_startup.get()
        self.chk_startup.configure(state="disabled")
        self.lbl_startup_status.configure(text="⏳ Menerapkan...", text_color="#EAB308")

        def worker():
            time.sleep(0.35)
            ok = api.set_startup(val)
            self.after(0, self._on_startup_completed, ok, val)

        threading.Thread(target=worker, daemon=True).start()

    def _on_startup_completed(self, success, val):
        self.chk_startup.configure(state="normal")
        if success:
            txt = "✓ Aktif di Startup" if val else "✓ Dinonaktifkan"
            self.lbl_startup_status.configure(text=txt, text_color=("#16A34A", "#4ADE80"))
        else:
            self.lbl_startup_status.configure(text="✗ Gagal menulis", text_color="#EF4444")
        self.after(2500, lambda: self.lbl_startup_status.configure(text=""))

    def _on_tray_toggle(self):
        val = self.var_tray.get()
        self.chk_tray.configure(state="disabled")
        self.lbl_tray_status.configure(text="⏳ Menerapkan...", text_color="#EAB308")

        def worker():
            time.sleep(0.3)
            self.config["run_in_tray"] = val
            api.save_preferences(self.config)

            global tray_icon
            if tray_icon:
                tray_icon.visible = val

            self.after(0, self._on_tray_completed, val)

        threading.Thread(target=worker, daemon=True).start()

    def _on_tray_completed(self, val):
        self.chk_tray.configure(state="normal")
        txt = "✓ Minimize aktif" if val else "✓ Langsung Exit"
        self.lbl_tray_status.configure(text=txt, text_color=("#16A34A", "#4ADE80"))
        self.after(2500, lambda: self.lbl_tray_status.configure(text=""))

    def _on_theme_change(self, choice=None):
        pass

    # ─── VIEW SWITCHING ───────────────────────────────────────────────
    def show_dashboard(self):
        self.view_smart.grid_forget()
        self.view_cleaner.grid_forget()
        self.view_settings.grid_forget()
        self.view_dashboard.grid(row=0, column=0, sticky="nsew")
        self.current_view = "dashboard"

    def show_smart_view(self):
        self.view_dashboard.grid_forget()
        self.view_cleaner.grid_forget()
        self.view_settings.grid_forget()
        self.view_smart.grid(row=0, column=0, sticky="nsew")
        self.current_view = "smart"
        self.populate_smart_view()

    def show_cleaner(self):
        self.view_dashboard.grid_forget()
        self.view_smart.grid_forget()
        self.view_settings.grid_forget()
        self.view_cleaner.grid(row=0, column=0, sticky="nsew")
        self.current_view = "cleaner"
        self._populate_cleaner_async()

    def show_settings(self):
        self.view_dashboard.grid_forget()
        self.view_smart.grid_forget()
        self.view_cleaner.grid_forget()
        self.view_settings.grid(row=0, column=0, sticky="nsew")
        self.current_view = "settings"

    # ─── HOMEPAGE IN-PLACE UPDATE & REALTIME TELEMETRY ────────────────
    def fast_update_dashboard(self, force=False):
        partitions = api.get_partitions()
        current_drives = set()
        is_dark = self.current_theme == "dark"

        for part in partitions:
            drv = part["drive"]
            current_drives.add(drv)

            if part["color_status"] == "critical":
                bar_color = "#EF4444"
            elif part["color_status"] == "warning":
                bar_color = "#EAB308"
            else:
                bar_color = "#FFFFFF" if is_dark else "#242426"

            free_txt = f"{int(part['free_gb'])} GB free" if part["free_gb"] >= 100 else f"{part['free_gb']:.1f} GB free"
            tot_txt = f"{int(part['total_gb'])} GB total"

            if drv in self.part_cards and not force:
                c = self.part_cards[drv]
                c["free_lbl"].configure(text=free_txt)
                c["bar"].set(part["percent_used"] / 100.0)
                c["bar"].configure(progress_color=bar_color)
            else:
                if drv in self.part_cards:
                    self.part_cards[drv]["frame"].destroy()

                p_card = ctk.CTkFrame(self.scroll_parts, fg_color=("gray95", "#18181A"), corner_radius=10, border_width=1, border_color=("#E5E5EA", "#242428"))
                p_card.pack(fill="x", pady=4, padx=2)

                p_row1 = ctk.CTkFrame(p_card, fg_color="transparent")
                p_row1.pack(fill="x", padx=10, pady=(8, 2))

                d_lbl = ctk.CTkLabel(p_row1, text=f"{drv}  {part['label']}", font=FONT_BOLD)
                d_lbl.pack(side="left")

                pct_lbl = ctk.CTkLabel(p_row1, text=f"{part['percent_used']}%", font=FONT_CAPTION, text_color="gray")
                pct_lbl.pack(side="right")

                p_bar = ctk.CTkProgressBar(p_card, height=6, corner_radius=3)
                p_bar.set(part["percent_used"] / 100.0)
                p_bar.configure(progress_color=bar_color)
                p_bar.pack(fill="x", padx=10, pady=4)

                p_row2 = ctk.CTkFrame(p_card, fg_color="transparent")
                p_row2.pack(fill="x", padx=10, pady=(2, 8))

                free_lbl = ctk.CTkLabel(p_row2, text=free_txt, font=(FONT_FAMILY, 11, "bold"))
                free_lbl.pack(side="left")

                tot_lbl = ctk.CTkLabel(p_row2, text=tot_txt, font=(FONT_FAMILY, 10), text_color="gray")
                tot_lbl.pack(side="right")

                self.part_cards[drv] = {
                    "frame": p_card,
                    "free_lbl": free_lbl,
                    "bar": p_bar,
                }

        for stale in list(self.part_cards.keys()):
            if stale not in current_drives:
                self.part_cards[stale]["frame"].destroy()
                del self.part_cards[stale]

        r_mb, w_mb = api.get_io_rate()
        self.lbl_io_read.configure(text=f"📥 Read: {r_mb:.1f} MB/s")
        self.lbl_io_write.configure(text=f"📤 Write: {w_mb:.1f} MB/s")

    def _start_refresh_loop(self):
        if not self.winfo_exists():
            return
        if self.is_visible and self.current_view == "dashboard":
            self.fast_update_dashboard()
        if self.winfo_exists():
            self.after(1000, self._start_refresh_loop)

    # ─── VISIBILITY & EXIT LOGIC ───────────────────────────────────────
    def on_close_clicked(self):
        if not self.config.get("run_in_tray", True):
            self.quit_app()
        else:
            self.toggle_display()

    def quit_app(self):
        global tray_icon
        if tray_icon:
            try:
                tray_icon.visible = False
                tray_icon.stop()
            except Exception:
                pass
        try:
            self.destroy()
        except Exception:
            pass
        if app_mutex:
            try:
                ctypes.windll.kernel32.CloseHandle(app_mutex)
            except Exception:
                pass
        os._exit(0)

    def toggle_display(self):
        if self.is_visible:
            self.withdraw()
            self.is_visible = False
        else:
            x, y = get_flyout_position()
            self.geometry(f"{WINDOW_WIDTH}x{WINDOW_HEIGHT}+{x}+{y}")
            self.deiconify()
            self.is_visible = True

            self.lift()
            self.attributes("-topmost", True)
            hwnd = self.winfo_id()
            ctypes.windll.user32.SetWindowPos(hwnd, -1, x, y, WINDOW_WIDTH, WINDOW_HEIGHT, 0x0040 | 0x0001)
            ctypes.windll.user32.SetForegroundWindow(hwnd)

            if self.current_view == "dashboard":
                self.fast_update_dashboard()


# ─── SYSTEM TRAY THREAD ───────────────────────────────────────────────
def setup_tray_thread():
    global tray_icon
    icon_img = load_brand_icon()

    def on_toggle(*_):
        if app_instance and app_instance.winfo_exists():
            app_instance.after(0, app_instance.toggle_display)

    def on_exit(*_):
        if app_instance and app_instance.winfo_exists():
            app_instance.after(0, app_instance.quit_app)
        else:
            if tray_icon:
                try:
                    tray_icon.stop()
                except Exception:
                    pass
            os._exit(0)

    menu = pystray.Menu(
        pystray.MenuItem("Show / Hide", on_toggle, default=True),
        pystray.MenuItem(f"{APP_NAME} v{APP_VERSION} - by {APP_PUBLISHER}", None, enabled=False),
        pystray.Menu.SEPARATOR,
        pystray.MenuItem("Exit", on_exit)
    )

    tray_icon = pystray.Icon(
        APP_NAME,
        icon_img,
        f"{APP_NAME} v{APP_VERSION} - by {APP_PUBLISHER}",
        menu
    )
    tray_icon.run()


# ─── INITIALIZATION LAUNCHER ──────────────────────────────────────────
def main():
    global app_instance

    # 1. Instantiate Main Window (Withdrawn initially)
    app_instance = MyDisklyApp()

    # 2. Splash Screen as CTkToplevel attached to app_instance (Shares 1 Tk Interp!)
    splash = SplashScreen(master=app_instance)
    splash.update_step(0.25, "Memuat modul disktrex.dll...")
    time.sleep(0.2)

    splash.update_step(0.55, "Menginisialisasi sensor penyimpanan & SMART...")
    api.get_smart_data()
    api.get_partitions()
    time.sleep(0.2)

    splash.update_step(0.85, "Mempersiapkan antarmuka MyDiskly...")
    time.sleep(0.15)

    splash.update_step(1.0, "Siap digunakan!")
    time.sleep(0.15)
    splash.destroy()

    # 3. Start System Tray
    threading.Thread(target=setup_tray_thread, daemon=True).start()

    # 4. Display main window if not launched with --silent
    if "--silent" not in sys.argv:
        app_instance.after(50, app_instance.toggle_display)

    # 5. Run GUI Loop
    app_instance.mainloop()


if __name__ == "__main__":
    main()
