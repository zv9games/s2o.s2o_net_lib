param (
    [string]$exePath
)

# Add DLL directory to the PATH environment variable
$dllDir = (Resolve-Path "src\s2o_dll").Path
$currentPath = [System.Environment]::GetEnvironmentVariable("PATH")
$newPath = "$dllDir;$currentPath"
[System.Environment]::SetEnvironmentVariable("PATH", $newPath, [System.EnvironmentVariableTarget]::Process)

$process = New-Object System.Diagnostics.ProcessStartInfo
$process.FileName = "powershell.exe"
$process.Arguments = "-Command & {Start-Process $exePath -ArgumentList '--admin' -Verb runAs -WindowStyle Hidden}"
$process.WindowStyle = "Hidden"

[System.Diagnostics.Process]::Start($process)
