# Irkle

A [blake3](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE3)-based merkle (hash) tree implementation for superfast trees ⚡

## Example

```rust
use irkle::Tree;

fn main() {
    println!("{:#?}", Tree::new(vec!["hello", "there"]));
}
```
