# Speak Playground

Run [Speak](https://github.com/speak-lang/speak) programs in your browser, in English or Kiswahili. The interpreter ([`speak-core`](https://github.com/speak-lang/speak)) is compiled to WebAssembly and runs entirely on your device, so nothing is sent anywhere.

Try it: <https://speak-lang.github.io/speak-playground/>
Website: <https://speak-lang.github.io/>

The playground shows the value of the last line of your program. `print` writes to a console that browsers do not give WebAssembly, so use `sprint` (Kiswahili: `mlolongo_andika`) to produce the text you want to see.

## Run it locally

The pinned toolchain (Rust 1.98.1 with the `wasm32-unknown-unknown` target) is installed automatically by `rust-toolchain.toml` if you use [rustup](https://rustup.rs/).

```sh
cargo install --locked trunk
trunk serve --port 8089
```

Then open <http://localhost:8089/>. `cargo test` checks the bundled samples natively.

## Rebuild the published site

GitHub Pages serves the `docs/` folder of the `main` branch (there is no CI), so rebuilt output is committed:

```sh
rm -rf docs
trunk build --release --public-url /speak-playground/ --dist docs
touch docs/.nojekyll
```

Commit the new `docs/` together with your change.

## Licence

See [LICENSE.md](LICENSE.md).
