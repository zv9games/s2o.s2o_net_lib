param (
    [string]$exePath
)

# Add DLL directory to the PATH environment variable
$dllDir = (Resolve-Path "src\windivert").Path
$currentPath = [System.Environment]::GetEnvironmentVariable("PATH")
$newPath = "$dllDir;$currentPath"
[System.Environment]::SetEnvironmentVariable("PATH", $newPath, [System.EnvironmentVariableTarget]::Process)

# Create a temporary script to execute elevated
$tempScriptPath = "$env:TEMP\elevatedScript.ps1"
@"
Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -NoExit -Command `"Write-Host Press Enter to close; Read-Host`"' -Verb RunAs -WindowStyle Normal
"@ | Out-File -FilePath $tempScriptPath -Encoding utf8

# Run this script directly
powershell.exe -File $tempScriptPath