# Dog Food For You

A mock repository to demonstrate various optimization techniques and dead ends while
building a recommendation service for dog food.

## To Run

### Install Rust

You can follow the installation steps in [The Rust Programming Language book](https://doc.rust-lang.org/book/ch01-01-installation.html)
to get started.

### Generating Data

To generate source files for the service, run the `generate_source_data` binary:

```sh
cargo run --release --bin generate_source_data
```

For options, use the `--help` flag:

```sh
cargo run --bin generate_source_data -- --help
```
