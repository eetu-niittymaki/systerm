# === Configuration ===
$repo = "eetu-niittymaki/systerm" 
$program = "systerm.exe"
$installDir = "$env:USERPROFILE\systerm"

# Create install directory if it doesn't exist
if (!(Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir | Out-Null }

$releaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"

# Find the zip asset
$asset = $releaseInfo.assets | Where-Object { $_.name -like "systerm.zip" }
if (-not $asset) {
    Write-Error "No zip found."
    exit 1
}

# Download the zip to temp
$tempZip = "$env:TEMP\$($asset.name)"
Write-Host "Downloading $($asset.browser_download_url)..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $tempZip

# Extract the zip to install directory
Write-Host "Extracting to $installDir..."
Expand-Archive -Path $tempZip -DestinationPath $installDir -Force

# Add install directory to PATH if not already
if (-not ($env:Path -split ";" | Where-Object { $_ -eq $installDir })) {
    [Environment]::SetEnvironmentVariable("Path", $env:Path + ";" + $installDir, [EnvironmentVariableTarget]::User)
    Write-Host "Added $installDir to PATH. Restart your terminal to use $program."
} else {
    Write-Host "$installDir is already in PATH."
}

Write-Host "Installation complete!"