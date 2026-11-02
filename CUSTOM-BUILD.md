# Custom offline compatibility build

Based on gerdaroot/Horizon-Offline-Launcher commit
`6f3d7b2e84989edff1c2d47ee9777c5e265e6097` (MIT; see LICENSE).

## Status

This is modified source, not a compiled or verified Windows release. No Rust,
MSVC C++ compiler or Windows SDK was available locally. Runtime tests and the
full build have not been run. Connection to FrostSMP has not been verified.

## Changes

- An offline client completes an encryption handshake instead of stopping at
  the local Microsoft sign-in message. The server still decides whether to
  allow the player. This does not create a Microsoft session and does not
  authenticate to an online-mode server. Signed-in login behavior is unchanged.
- During Prepare, an existing client jar must match the pinned Minecraft SHA-1.
  The launcher then downloads missing assets from Mojang into its own
  `offline-assets/<version>` cache. Indexes and objects retain SHA-1 validation;
  finished objects are reused and corrupt ones are retried. The existing jar is
  copied locally, not downloaded. Libraries are linked read-only by convention
  for the preparer. Other launchers' files are not repaired in place.
- When asset repair fails without cancellation, preparation uses the existing
  install and logs the error. Run Prepare again to retry. Assets must be cached
  before fully offline use. A valid local Minecraft installation is still needed.

## Build on GitHub Actions

1. Put this source into a repository you control. This package has not created
   or published a GitHub repository for you.
2. Open Actions -> Custom Windows build (no publishing) -> Run workflow.
3. The workflow runs the launcher tests and an encrypted-login regression test,
   then builds both executables. If a step fails, inspect its log; there is no
   pre-existing passing CI result for these changes.
4. Download the `horizon-custom-windows-x64` artifact after a successful run.

The workflow uploads build artifacts only; it creates no public release.
It may take a long time on a cold build and consumes your Actions allowance.

## Build locally

Install stable Rust and Visual Studio Build Tools with Desktop development with
C++ and a Windows SDK. Make `fxc.exe` available as `GPUI_FXC_PATH` if required.
From this source directory, run:

```powershell
cargo test --locked --release -p horizon-client --lib net::tests::lan_login_continues_after_authentication_failure
cargo test --locked --release --manifest-path launcher/Cargo.toml
cargo build --locked --release -p horizon-client --bin horizon-client
cargo build --locked --release --manifest-path launcher/Cargo.toml --bin horizon-launcher
```

Copy `target/release/horizon-client.exe`,
`launcher/target/release/horizon-launcher.exe` and `Start-Custom.ps1` together.
The upstream build.ps1 builds the launcher only; multiplayer needs the custom
client as well.

## Try it with your existing install

Back up your saves before trying an experimental build. Run Start-Custom.ps1.
In Versions, choose Import a game build and select the custom horizon-client.exe.
Use your existing prepared game files of the matching version, then select that
imported build. Importing keeps the custom client separate from downloaded game
updates. The binaries alone contain no prepared Minecraft resources.

Choose your existing Minecraft jar/install and run Prepare to populate assets.
Try the same server address and offline player name that work in your Java
client. If the server refuses the connection, keep its exact disconnect message
and the game log. Version translation, proxy behavior and server authentication
rules can still prevent a connection; this change does not guarantee FrostSMP
support or improve rendering performance.

## Added regression coverage (not yet executed)

The network fixture validates the RSA challenge response, enables encryption,
and checks that a server's rejection reaches the client. Asset tests cover no
local jar, no game download, untouched source assets, cache reuse, corruption
repair and resuming after a corrupt download.
