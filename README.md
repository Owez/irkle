# Irkle

[![Tests](https://github.com/Owez/irkle/workflows/Tests/badge.svg)](https://github.com/Owez/irkle/actions?query=workflow%3ATests)
[![Docs](https://docs.rs/irkle/badge.svg)](https://docs.rs/irkle)

A merkle tree library using [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3) and [binary tree arrays](https://en.wikipedia.org/wiki/Binary_tree#Arrays) for superfast trees ⚡

## Example

***Coming soon***

## Installation

Simply add the following to your `Cargo.toml` file:

```toml
[depedencies]
irkle = "0.1" # NOTE: not released just yet!
```

## Objectives

This project has explicit objectives that are stuck to whilst developing and maintaining:

- __High preformance__
- Heavily documented functions, should read as semi-documentation, semi-tutorial on merkle trees
- Low-as-possible dependency count, only current dependency is the `blake3` hasing library
