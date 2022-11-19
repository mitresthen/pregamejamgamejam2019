# Bergen Game Jam - Rust Game Engine

## Map Editor

To run Map Editor for your level (e.g. `Basement.json`), while at `.\engine` run:

`cargo run --example level_editor ..\assets\levels\Basement.json`

For other levels change the filename to what you desire.

### Building Map Editor on WSL to run it on Windows

On WSL run this command while in `.\engine` directory:

`cargo build --release --target x86_64-pc-windows-gnu --example level_editor`

Once that's done you need to copy `SDL2.dll` from `.\engine\target\x86_64-pc-windows-gnu\release` to `.\engine\target\x86_64-pc-windows-gnu\release\examples`.

Then you can open it from Windows Terminal by running:

`.\engine\target\x86_64-pc-windows-gnu\release\examples\level_editor.exe .\assets\levels\Basement.json`

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
