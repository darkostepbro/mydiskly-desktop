"""
MyDiskly Build Tool - Standalone Executable Bundler
Author: Darkostepbro
Repository: https://github.com/darkostepbro/mydiskly-desktop

Builds the complete production portable package into dist/mydiskly-v1/.
"""

import os
import sys
import shutil
import subprocess

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC_MAIN = os.path.join(REPO_ROOT, "src", "main.py")
ICON_ICO = os.path.join(REPO_ROOT, "assets", "icon.ico")
DIST_BASE = os.path.join(REPO_ROOT, "dist")
BUILD_TEMP = os.path.join(REPO_ROOT, "build")
TARGET_DIST = os.path.join(DIST_BASE, "mydiskly-v1")

def build_all():
    print("=" * 60)
    print("MyDiskly v1.0.2026 - Packaging Pipeline")
    print("Owner: Darkostepbro | https://github.com/darkostepbro/mydiskly-desktop")
    print("=" * 60)

    # 1. Ensure disktrex.dll is compiled
    dll_path = os.path.join(REPO_ROOT, "disktrex.dll")
    build_dll_script = os.path.join(REPO_ROOT, "scripts", "build_dll.py")
    res = subprocess.run([sys.executable, build_dll_script, dll_path])
    if res.returncode != 0:
        print("[ERROR] Failed to compile disktrex.dll")
        sys.exit(1)

    # 2. Run PyInstaller
    temp_dist = os.path.join(REPO_ROOT, "temp_dist")
    cmd = [
        sys.executable, "-m", "PyInstaller",
        "--noconsole",
        "--onedir",
        "--contents-directory", "lib",
        "--name", "MyDiskly",
        f"--icon={ICON_ICO}",
        "--clean",
        "--hidden-import", "configparser",
        "--hidden-import", "json",
        "--hidden-import", "winreg",
        "--hidden-import", "subprocess",
        "--hidden-import", "datetime",
        "--distpath", temp_dist,
        "--workpath", BUILD_TEMP,
        "--collect-all", "customtkinter",
        "--collect-all", "pystray",
        "--collect-all", "psutil",
        "--collect-all", "PIL",
        SRC_MAIN
    ]

    print("\n[1/4] Running PyInstaller bundler...")
    res = subprocess.run(cmd)
    if res.returncode != 0:
        print("[ERROR] PyInstaller bundling failed.")
        sys.exit(1)

    # 3. Assemble target directory
    print("\n[2/4] Assembling portable release distribution...")
    if os.path.exists(TARGET_DIST):
        shutil.rmtree(TARGET_DIST, ignore_errors=True)
    os.makedirs(TARGET_DIST, exist_ok=True)

    built_pkg = os.path.join(temp_dist, "MyDiskly")

    # Copy MyDiskly.exe
    shutil.copy2(os.path.join(built_pkg, "MyDiskly.exe"), os.path.join(TARGET_DIST, "MyDiskly.exe"))

    # Copy lib directory
    shutil.copytree(os.path.join(built_pkg, "lib"), os.path.join(TARGET_DIST, "lib"))

    # Copy disktrex.dll
    shutil.copy2(dll_path, os.path.join(TARGET_DIST, "disktrex.dll"))

    # Copy mydiskly.ini
    shutil.copy2(os.path.join(REPO_ROOT, "mydiskly.ini"), os.path.join(TARGET_DIST, "mydiskly.ini"))

    # Copy assets
    assets_target = os.path.join(TARGET_DIST, "assets")
    shutil.copytree(os.path.join(REPO_ROOT, "assets"), assets_target)

    # Copy app_icon.png to root for flyout & tray icon fallback
    app_icon_src = os.path.join(REPO_ROOT, "assets", "app_icon.png")
    if os.path.exists(app_icon_src):
        shutil.copy2(app_icon_src, os.path.join(TARGET_DIST, "app_icon.png"))

    # 4. Clean build artifacts
    print("[3/4] Cleaning temporary work directories...")
    shutil.rmtree(temp_dist, ignore_errors=True)
    shutil.rmtree(BUILD_TEMP, ignore_errors=True)
    spec_file = os.path.join(REPO_ROOT, "MyDiskly.spec")
    if os.path.exists(spec_file):
        os.remove(spec_file)

    print("\n[4/4] Release Package Ready at:")
    print(f"      {TARGET_DIST}")
    print("=" * 60)

if __name__ == "__main__":
    build_all()
