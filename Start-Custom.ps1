# Run beside the custom launcher. It prevents launcher self-updates for this run.
# Uses the bundled custom client with resources prepared in the launcher.
$ErrorActionPreference = 'Stop'
$launcher = Join-Path $PSScriptRoot 'horizon-launcher.exe'
if (-not (Test-Path -LiteralPath $launcher)) {
    throw 'Build/download the custom executables first; see CUSTOM-BUILD.md.'
}
$env:HORIZON_LAUNCHER_NO_SELF_UPDATE = '1'
$env:HORIZON_USE_BUNDLED_CLIENT = '1'
Start-Process -FilePath $launcher
