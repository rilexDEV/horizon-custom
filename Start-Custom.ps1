# Run beside the custom launcher. It prevents launcher self-updates for this run.
# Import the custom client with Versions -> Import a game build before playing.
$ErrorActionPreference = 'Stop'
$launcher = Join-Path $PSScriptRoot 'horizon-launcher.exe'
if (-not (Test-Path -LiteralPath $launcher)) {
    throw 'Build/download the custom executables first; see CUSTOM-BUILD.md.'
}
$env:HORIZON_LAUNCHER_NO_SELF_UPDATE = '1'
Start-Process -FilePath $launcher
