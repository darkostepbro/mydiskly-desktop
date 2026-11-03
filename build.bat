@echo off
rem MyDiskly - Automated Release Build Script (Batch / CMD)
rem Builds all native binaries in proper sequence and syncs Uninstall.exe

echo ==========================================================
echo   MyDiskly v1.1.2026 - Automated Release Build Pipeline   
echo ==========================================================

rem Ensure MinGW/LLVM toolchain is in PATH if present
if exist "C:\llvm-mingw-20261020-ucrt-x86_64\bin\gcc.exe" (
    set "PATH=C:\llvm-mingw-20261020-ucrt-x86_64\bin;%PATH%"
)

echo.
echo [1/3] Compiling MyDiskly Desktop Application (MyDiskly.exe)...
cargo build --release -p mydiskly-app
if errorlevel 1 (
    echo Error: Failed to compile mydiskly-app.
    exit /b %errorlevel%
)

echo.
echo [2/3] Compiling MyDiskly Setup Wizard & Installer (MyDiskly-Setup.exe)...
cargo build --release -p mydiskly-installer
if errorlevel 1 (
    echo Error: Failed to compile mydiskly-installer.
    exit /b %errorlevel%
)

echo.
echo [3/3] Synchronizing Standalone Uninstaller Suite (Uninstall.exe)...
copy /Y "target\release\MyDiskly-Setup.exe" "target\release\Uninstall.exe" >nul
if errorlevel 1 (
    echo Error: Failed to copy Uninstall.exe.
    exit /b %errorlevel%
)

echo.
echo ==========================================================
echo Build Succeeded! Compiled binaries are in target\release\
echo ==========================================================
dir target\release\MyDiskly.exe target\release\MyDiskly-Setup.exe target\release\Uninstall.exe
