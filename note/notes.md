1.67.1 (2023-02-09)
1.80.1 (2024-08-08)

--------------------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
--------------------------------
cargo --version
rustc --version
rustup update
rustup self uninstall
--------------------------------

cargo new hello-rust
cargo new --vcs=git
cargo new --help
cargo doc --open
cargo tree

cargo add ferris-says

cargo build
cargo build --release

rustc main.rs

cargo run
tree .

rustup component add rustfmt
`rustfmt instead`
`reformat on save`

#### new project

```shell
cargo new guessing_game
$ cd guessing_game
```

```shell
$ cargo update         # updates all dependencies
$ cargo update regex   # updates just “regex”
```
