# Irkle

A [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based merkle (hash) tree implementation for superfast trees ⚡ 

- [Documentation](https://docs.rs/irkle)

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
