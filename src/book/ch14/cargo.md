### cargo & crates

https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
https://doc.rust-lang.org/cargo/index.html
https://doc.rust-lang.org/cargo/reference/profiles.html
https://doc.rust-lang.org/cargo/commands/cargo-tree.html

#### all the libraries at:

https://crates.io

### build (with debug)

```shell
cargo build
```

### build (without debug + optimization)

```shell
cargo build --release
```

`cargo.toml`:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

result are
in `targed/debug`
and `targed/release`

`cargo publish`

### install runnable as binary

`cargo install`
