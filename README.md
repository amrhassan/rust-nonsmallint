[![Build Status](https://travis-ci.org/amrhassan/rust-nonsmallint.svg?branch=master)](https://travis-ci.org/amrhassan/rust-nonsmallint) [![crates.io](https://img.shields.io/crates/v/nonsmallnum.svg)](https://crates.io/crates/nonsmallnum)

# rust-nonsmallint #
Basic arithmetic for unsigned arbitrarily-sized integers in pure Rust

# Usage #
Add it to your project dependencies in `Cargo.toml`:
```toml
[dependencies]
nonsmallnum = "0.0.5"
```

# Examples #
```rust
extern crate nonsmallnum;
use nonsmallnum::NonSmallInt;

fn main() {
    let x = NonSmallInt::parse("4236523").expect("Failed to parse a non-small number");
    let y = NonSmallInt::of(43);

    println!("{} + {} == {}", x, y, &x + &y);
    println!("{} - {} == {}", x, y, &x - &y);
    println!("{} * {} == {}", x, y, &x * &y);
    println!("{} / {} == {}", x, y, &x / &y);
    println!("{} % {} == {}", x, y, &x % &y);
    println!("{} < {} == {}", x, y, &x < &y);
}
```

# API Doc #
You can find API doc of the latest release [here](https://amrhassan.github.io/rust-nonsmallnum/nonsmallnum/).
