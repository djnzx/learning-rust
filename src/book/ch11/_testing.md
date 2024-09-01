## testing notes

- [basics](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [running](https://doc.rust-lang.org/book/ch11-02-running-tests.html)
- [organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)

### test code is automatically created during new project creation with `--lib`

```shell
cargo new adder --lib
```

### run tests (ignored excluded)

```shell
cargo test
```

> test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

### run tests (ignored only)

```shell
cargo test -- --ignored
```

> test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

### run tests (normal + ignored)

```shell
cargo test -- --include-ignored
```

> test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

- `#[cfg(test)]` to compile and run during `cargo test` only
-

### run specific tests, contains semantics

```shell
cargo test test-name
```

`test-name1`, `test-name2` will match and run

### run specific tests, exact equals semantics

```shell
cargo test mod-name::test-name -- --exact
```

exact match with fully qualified module

```shell
cargo test -- --show-output
```

allow to see output (by default switched off)
