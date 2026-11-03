# MyDiskly - Automated Release Build Script (PowerShell)
# Builds all native binaries in proper sequence and syncs Uninstall.exe

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  MyDiskly v1.1.2026 - Automated Release Build Pipeline   " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Ensure MinGW/LLVM toolchain is in PATH if present
$possiblePaths = @(
    "C:\llvm-mingw-20261020-ucrt-x86_64\bin",
    "C:\llvm-mingw\bin",
    "C:\msys64\mingw64\bin",
    "C:\Program Files\LLVM\bin"
)
foreach ($p in $possiblePaths) {
    if (Test-Path $p) {
        if ($env:PATH -notlike "*$p*") {
            $env:PATH = "$p;$env:PATH"
        }
        break
    }
}

# Step 1: Compile Main Application
Write-Host "`n[1/3] Compiling MyDiskly Desktop Application (MyDiskly.exe)..." -ForegroundColor Yellow
cargo build --release -p mydiskly-app
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to compile mydiskly-app. Build aborted."
    exit $LASTEXITCODE
}

# Step 2: Compile Installer Suite (embeds the freshly built MyDiskly.exe)
Write-Host "`n[2/3] Compiling MyDiskly Setup Wizard & Installer (MyDiskly-Setup.exe)..." -ForegroundColor Yellow
cargo build --release -p mydiskly-installer
if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to compile mydiskly-installer. Build aborted."
    exit $LASTEXITCODE
}

# Step 3: Synchronize Uninstaller Suite
Write-Host "`n[3/3] Synchronizing Standalone Uninstaller Suite (Uninstall.exe)..." -ForegroundColor Yellow
Copy-Item "target/release/MyDiskly-Setup.exe" "target/release/Uninstall.exe" -Force

Write-Host "`nBuild Succeeded! Compiled binaries in target/release/:" -ForegroundColor Green
Get-Item "target/release/MyDiskly.exe", "target/release/MyDiskly-Setup.exe", "target/release/Uninstall.exe" | Select-Object Name, Length, LastWriteTime | Format-Table -AutoSize
