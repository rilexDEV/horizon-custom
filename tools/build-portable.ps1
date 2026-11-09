param(
    [string]$HorizonRevision,
    [ValidateRange(1, 32)][int]$Jobs = 2,
    [string]$MixinsRevision = 'a2af5c0dd4117d51ecd70066ba7a00b4a2c10dab'
)
$ErrorActionPreference = 'Stop'
$env:CARGO_BUILD_JOBS = [string]$Jobs
$root = Split-Path $PSScriptRoot -Parent
Set-Location $root
if (-not $env:RUSTFLAGS) { $env:RUSTFLAGS = '-C target-feature=+crt-static' }
function Run-Cargo { & cargo @args; if ($LASTEXITCODE -ne 0) { throw "cargo failed: $args" } }
if (-not $HorizonRevision) {
    if (Test-Path (Join-Path $root ".git")) {
        $HorizonRevision = & git rev-parse HEAD
        if ($LASTEXITCODE -ne 0) { throw "Cannot read source commit" }
    } else {
        $sourceNote = Get-Content (Join-Path $root 'CUSTOM-BUILD.md') -Raw
        $baseCommit = [regex]::Match($sourceNote, 'commit\s+`([0-9a-f]{40})`')
        if (-not $baseCommit.Success) { throw 'Supply -HorizonRevision with the full source commit.' }
        $HorizonRevision = $baseCommit.Groups[1].Value
        Write-Warning 'No Git metadata: using the documented upstream base for native-mod source resolution.'
    }
}
if ($HorizonRevision -notmatch '^[0-9a-f]{40}$' -or $MixinsRevision -notmatch '^[0-9a-f]{40}$') { throw 'Use full 40-character commit hashes.' }
if (-not $env:GPUI_FXC_PATH) {
    $fxc = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin" -Filter fxc.exe -Recurse |
        Where-Object { $_.Directory.Name -eq 'x64' } | Sort-Object FullName -Descending | Select-Object -First 1
    if (-not $fxc) { throw 'Install the Windows SDK and Visual Studio C++ Build Tools.' }
    $env:GPUI_FXC_PATH = $fxc.FullName
}
Run-Cargo build --locked --release -p horizon-client --bin horizon-client
Run-Cargo build --locked --release -p horizon-prepare --bin horizon-prepare
$toolRoot = Join-Path $root "target/portable-tools/$MixinsRevision"
Run-Cargo install --locked --git https://github.com/horizon-rust/horizon-mixins --rev $MixinsRevision --root $toolRoot horizon-mixins-cli
Add-Type -AssemblyName System.IO.Compression.FileSystem
$payload = Join-Path $root ("target/portable-" + [guid]::NewGuid().ToString('N') + '.zip')
$zip = [IO.Compression.ZipFile]::Open($payload, [IO.Compression.ZipArchiveMode]::Create)
function Add-PayloadFile([string]$file, [string]$name) {
    [IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, $file, $name, [IO.Compression.CompressionLevel]::Optimal) | Out-Null
}
try {
    Add-PayloadFile (Join-Path $root 'target/release/horizon-client.exe') 'horizon-client.exe'
    Add-PayloadFile (Join-Path $root 'target/release/horizon-prepare.exe') 'horizon-prepare.exe'
    Add-PayloadFile (Join-Path $toolRoot 'bin/horizon-mixins.exe') 'horizon-mixins.exe'
    foreach ($name in @('research-lock.json','native-hooks.json','LICENSE','THIRD_PARTY_NOTICES.md')) { Add-PayloadFile (Join-Path $root $name) $name }
    foreach ($folder in @('crates/horizon-prepare','mods/horizon/src/test/java')) {
        Get-ChildItem (Join-Path $root $folder) -Recurse -File | Where-Object { $_.Extension -in '.rs','.toml','.java' } | ForEach-Object {
            $relative = [IO.Path]::GetRelativePath($root, $_.FullName).Replace('\','/')
            Add-PayloadFile $_.FullName $relative
        }
    }
    $meta = $zip.CreateEntry('portable.json')
    $writer = [IO.StreamWriter]::new($meta.Open())
    try { $writer.Write((@{horizon_revision=$HorizonRevision;mixins_revision=$MixinsRevision} | ConvertTo-Json)) } finally { $writer.Dispose() }
} finally { $zip.Dispose() }
$env:HORIZON_PORTABLE_PAYLOAD = $payload
Run-Cargo build --locked --release --manifest-path launcher/Cargo.toml --features portable --bin horizon-launcher
New-Item -ItemType Directory -Force (Join-Path $root 'dist') | Out-Null
Copy-Item (Join-Path $root 'launcher/target/release/horizon-launcher.exe') (Join-Path $root 'dist/HORIZON CUSTOM BY RILEXezz.exe') -Force
Write-Host 'Built dist/HORIZON CUSTOM BY RILEXezz.exe'
