# Custom Horizon build

Based on Horizon Offline Launcher. Original copyright and MIT notices are retained.

## Use

Extract the complete Windows artifact and run Start-Custom.ps1. Keep both executables together.
The launcher uses the bundled custom client when started through this script.
Select an installed Horizon version, choose the path to your existing matching Minecraft
installation or jar, and select Prepare. Native Horizon preparation does not download the
client jar or repair missing assets from Mojang. Supply a complete local installation.
Existing Java-instance download features are separate from native Horizon preparation.
Choose a valid profile name in the launcher before starting multiplayer. Signed-in accounts
use their own profile names. Online-mode servers still require a valid Microsoft session.

## Validation

GitHub Actions runs launcher tests and the encrypted-login regression test before building
both Windows executables. Build success does not establish compatibility with every server.
Game assets are not included in the distribution. See LICENSE and THIRD_PARTY_NOTICES.md.
