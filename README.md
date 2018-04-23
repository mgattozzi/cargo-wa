# Unsupported

Right now I have quite a few things going on and I'm more involved with the wasm-wg group currently.
I originally made this as a way to wrap the need to call `--target=wasm32-unknown-unknown` each
time. While you can still use this as is, I won't be releasing fixes or taking prs anytime soon as
this project is just not a priority for me right now.

# cargo-wasm

A cargo subcommand for working with Rust wasm projects!

See https://github.com/rust-lang-nursery/rust-wasm for the manual steps if cargo-wasm doesn't work for you.

## Build requirements

You'll need the latest stable version of `rustc`, `rustup`, and `cargo`
installed already.

## How to install cargo-wasm

```bash
# For the latest unstable version
cargo install --git https://github.com/mgattozzi/cargo-wasm
# For the version released to crates.io
cargo install cargo-wasm
```

That's all you need to do! Then you can start running commands!

## How to use cargo-wasm

### Setup

If you have never setup `rustup` for wasm or `wasm-gc` yet at all you need to run:

```bash
cargo wasm setup
```

This will install `wasm-gc` for you as well as setting up rustup to use the
`wasm32` backend.

### New Project

To start a new wasm project run:

```bash
cargo wasm new <project_name>
```

This will setup a project with a bare wasm skeleton to run wasm function from an
`index.html` file under the `site` folder.

### Build the Project

This command assumes you are at the project root. To build a wasm project run:

```bash
cargo wasm build
```

All builds are currently built/run in release mode due to a wasm bug in debug
builds. See issue #1.

### Run the Project

This command assumes you are at the project root. To run a wasm project run:

```bash
cargo wasm run
```

This will try to open your default browser and run the code from there.

All builds are currently built/run in release mode due to a wasm bug in debug
builds. See issue #1.

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
