# pregamejamgamejam2019

## Building

While those are installed you simply run:

`cargo build`

`cargo run`

To build the release version - you'll need a `cargo-script` package installed:

`cargo install cargo-script`

`cargo build --release`

`cargo script final` or `cargo script final --clear-cache`

### Windows

To build this game on Windows - you will need a preinstalled CMake and Visual Studio Build Tools.

## Map Editor

To run Map Editor for your level (e.g. `Basement.json`), while at `.\engine` run:

`cargo run --example level_editor ..\assets\levels\Basement.json`

For other levels change the filename to what you desire.
