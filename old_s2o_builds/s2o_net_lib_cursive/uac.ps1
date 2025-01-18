

# Check if the script is running with elevated permissions
if (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    if ($myInvocation.MyCommand.Path -eq $PSCommandPath) {
        Start-Process powershell -ArgumentList "-File", "`"$PSCommandPath`"" -Verb RunAs
        Exit
    }
}

# Set the correct working directory
$InstallDir = $env:S2O_INSTALL_PATH
if (-not $InstallDir) {
    $InstallDir = "C:\S2O\s2o_net_lib"  # Fallback
}
Set-Location -Path $InstallDir

# Check if this is the admin session or the initial session
if (-Not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Start-Process "cargo" -ArgumentList "run --release" -NoNewWindow -Wait
} else {
    $CurrentPID = [System.Diagnostics.Process]::GetCurrentProcess().Id
    Start-Process "cargo" -ArgumentList "run --release -- admin $CurrentPID" -NoNewWindow -Wait
}

# Keep this PowerShell window open so the session stays alive
Pause

# Stop logging to file
Stop-Transcript