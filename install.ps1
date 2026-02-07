# LinuxTeasing Installer for Windows
# Copilot Skill: rules/windows-install

Write-Host "🐧 Installing LinuxTeasing..." -ForegroundColor Cyan

# 1. Build Release
Write-Host "Building release binary..."
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed!"
    exit 1
}

# 2. Create Target Directory
$TargetDir = "$HOME\AppData\Local\Programs\LinuxTeasing"
if (-not (Test-Path -Path $TargetDir)) {
    New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null
}

# 3. Copy Binary
Copy-Item -Path "target\release\linux-teasing.exe" -Destination "$TargetDir\linux-teasing.exe" -Force
Write-Host "Binary installed to $TargetDir" -ForegroundColor Green

# 4. Add to PATH (Current Session + Persistent)
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$TargetDir*") {
    Write-Host "Adding to PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$TargetDir", "User")
    $env:Path += ";$TargetDir"
    Write-Host "Added to User PATH. You may need to restart your terminal." -ForegroundColor Yellow
} else {
    Write-Host "Already in PATH." -ForegroundColor Gray
}

# 5. Add to PowerShell Profile (Optional - for startup execution)
# The user might not want this hard forced, but the requirement is "terminal startup tool"
$ProfilePath = $PROFILE
if (-not (Test-Path $ProfilePath)) {
    New-Item -Type File -Path $ProfilePath -Force | Out-Null
}

$Command = "linux-teasing.exe"
$ProfileContent = Get-Content $ProfilePath -Raw -ErrorAction SilentlyContinue
if ($ProfileContent -notlike "*$Command*") {
    Add-Content -Path $ProfilePath -Value "`n# LinuxTeasing Judgment`n$Command"
    Write-Host "Added startup command to PowerShell Profile ($ProfilePath)" -ForegroundColor Green
} else {
    Write-Host "Startup command already in PowerShell Profile." -ForegroundColor Gray
}

Write-Host "🐧 Installation Complete. Stay strictly on UTC." -ForegroundColor Cyan
