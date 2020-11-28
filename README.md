# Irkle

[![Tests](https://github.com/Owez/irkle/workflows/Tests/badge.svg)](https://github.com/Owez/irkle/actions?query=workflow%3ATests)
[![Docs](https://docs.rs/irkle/badge.svg)](https://docs.rs/irkle)

> NOTE: This branch is outdated and is not being worked on, in favour of the `array-redo` branch which brings many preformance improvements. This branch represents the status of the `0.1.0-pre.1` verson of irkle

A [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based merkle (hash) tree implementation for superfast trees ⚡ 

## Example

```rust
use irkle::Tree;

fn main() {
    println!("{:#?}", Tree::new(vec!["hello", "there"]));
}
```

## Installation

Simply add the following to your `Cargo.toml` file:

```toml
[depedencies]
irkle = "0.1"
```

## Current status

Creation and verification of merkle trees have been implemented and are fast, but could be greatly improved with the help of parallelization as currently this library is single-threaded.

This is the tracking issue for the multi-threading of irkle: [`https://github.com/Owez/irkle/issues/1`](https://github.com/Owez/irkle/issues/1)
