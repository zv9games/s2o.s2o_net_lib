param (
    [string]$exePath
)

$process = New-Object System.Diagnostics.ProcessStartInfo
$process.FileName = "powershell.exe"
$process.Arguments = "-Command & {Start-Process $exePath -ArgumentList '--admin' -Verb runAs -WindowStyle Hidden}"
$process.WindowStyle = "Hidden"

[System.Diagnostics.Process]::Start($process)