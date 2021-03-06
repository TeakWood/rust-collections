# rust-collections
Learn Rust by implementing collections.

I was inspired by [Jon Gjengset's](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ) stream on implementing `hashmap` in rust. I decided to implement other collections myself

HashMap:
This is implemented following Jon's stream. Run
```rust
cargo r --example hashmap_demo
```

HashSet:
Simplified version that uses `std::collections::hash_map::DefaultHasher` for hashing key. Standard Hashset implementations internally use HashMap but I decided to implement everything from stratch
```rust
cargo r --example hashset_demo
```

LinkedList:
I wanted to use same struct signature as collections crate implementation. This taught me a lot on dealing with Box type and raw pointers. Also added implementation for Display trait for LinkedList. To run this lib we need nightly build features.
```rust
cargo +nightly r --example linkedlist_demo
```