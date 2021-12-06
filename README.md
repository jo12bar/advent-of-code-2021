# Advent of Code 2021

This repository contains my solutions for the [Advent of Code 2021][aoc2021] challenges. They are
primarily written in [Rust][rust-lang], purely because I like messing around with that programming
language and I want to familiarize myself with it more. I've been dealing with a lot of complex
C++ code at work recently, so I'm treating this as therapy.

## Building

### ...everything

In debug mode:

```shell
cargo build
```

In release mode:

```shell
cargo build --release
```

### ...just a single challenge

In debug mode:

```shell
cargo build --bin 2021_12_01_c2
```

In release mode:

```shell
cargo build --release --bin 2021_12_01_c2
```

## Running

In debug mode:

```shell
cargo run --bin 2021_12_01_c2
```

In release mode:

```shell
cargo run --release --bin 2021_12_01_c2
```

Alternatively, after [building](#Building) each challenge you can run the executables directly.
They should be located in the `target/debug` directory for debug builds, and `target/release` for
release buils.

## Executable naming

Executables are named according to the date of the challenge and the number of the challenge:

> **YYYY**\_**MM**\_**DD**\_C**N**

[aoc2021]: https://adventofcode.com/2021
[rust-lang]: https://www.rust-lang.org/
