# Bergen Game Jam - Rust Game Engine

## Map Editor

To run Map Editor for your level (e.g. `Basement.json`), while at `.\engine` run:

`cargo run --example level_editor ..\assets\levels\Basement.json`

For other levels change the filename to what you desire.

## Building

While you have rust and cargo installed you simply run:

`cargo build`

or

`cargo run`

### Building release version for Bergen Game Jam 2019

To build the release version - you'll need a `cargo-script` package installed:

`cargo install cargo-script`

`cargo build --release`

`cargo script final` or `cargo script final --clear-cache`
