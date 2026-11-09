# Portable custom launcher

This source now supports a single distributed Windows EXE. It contains the native
client, preparer, pinned mixin tool, original exporter Java sources, preparation
sources, lock metadata and licence notices. On first run it extracts these into
an immutable, content-addressed version under the launcher data directory.
Extraction is required to execute native child programs on Windows. No adjacent
PS1 script, client EXE or user-set environment variable is needed.

The portable client is selected on a new installation. Existing selections,
including native-mod builds, remain selected. Select the `portable-...` version
explicitly on an existing installation to use the embedded client. Portable
launchers disable upstream self-replacement so an upstream update cannot discard
the custom packaging. Download a new portable release to update the launcher.

Automatic Search beside Choose jar scans Downloads, Desktop, known Minecraft,
PrismLauncher and TLauncher folders in the background. It validates candidate
JARs against the selected version's pinned SHA-1. Search drive opens a folder
picker: select C:\, D:\ or any subtree. Permission failures are skipped; links
and Windows reparse points are not followed. Cancel stops the worker. The found
path is saved as the existing Minecraft JAR setting.

Minecraft JARs/assets are NOT redistributed. A legitimate local client JAR is
still needed. Preparation can download Java and missing resources. Native mod
compilation can download Rust, source and dependencies; this is not a fully
offline distribution of every possible build dependency. A differently pinned
mixin tool is installed through the existing toolchain manager.

## Local build (PowerShell 7)

Prerequisites: stable Rust MSVC, Visual Studio C++ Build Tools, Windows SDK,
Git and internet for initial dependencies. From the repository root:

    pwsh -File tools/build-portable.ps1 -Jobs 1

With a Git checkout, this uses HEAD. This local Desktop copy has no .git
directory, so the script falls back to the documented upstream base commit
in CUSTOM-BUILD.md for native-mod source resolution. Override with
-HorizonRevision FULL_COMMIT when appropriate.
Do not use a fabricated commit: native mods use that revision to obtain sources.
The script builds the client/preparer/tool first, creates the embedded payload,
and builds the launcher with its portable feature and static C runtime.
Output: dist/HORIZON CUSTOM BY RILEXezz.exe

Double-click that EXE. No Start-Custom.ps1 is needed. On a fresh installation,
select Automatic Search, then Prepare/Play as usual. After building native mods,
keep the native build selected; do not switch back to the portable base client.

## GitHub release

Commit the source in your own GitHub repository. The build-launcher.yml workflow
runs manually for an artifact, or publishes an EXE release for portable-v* tags:

    git tag portable-v1.0.0
    git push origin portable-v1.0.0

The release asset is named HORIZON CUSTOM BY RILEXezz.exe. Your website can link
to the asset on that repository's release page. No release is published merely
by editing this local folder.

## Validation

The launcher tests run in the Windows workflow before packaging. The workflow
also launches the finished EXE with --version in an isolated directory and
checks first-run extraction before uploading or releasing it. The jar-search
unit test covers recursive discovery, rejection of a misleading filename/hash,
and cancellation. Full game preparation and graphical startup need a Windows
run with the user's legally obtained JAR. Do not claim those passed from a
successful compiler check alone.

## Local verification result

Normal and portable-feature cargo check --tests passed. Isolated tests passed
for recursive hash-validated JAR discovery/cancellation and portable extraction,
cache reuse, preservation of native selection and exact tool revision matching.
PowerShell packaging syntax passed. The complete release test build was stopped
because of local memory pressure before tests executed. The final distributable
EXE and graphical startup have not been built/tested locally. CI runs the full
suite and a first-run extraction smoke test before publishing.
