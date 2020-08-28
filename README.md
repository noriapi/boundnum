[![Workflow Status](https://github.com/tanakh/hogehoge/workflows/Rust/badge.svg)](https://github.com/tanakh/hogehoge/actions?query=workflow%3A%22Rust%22)

# boundnum

This crate provides a type boundary and a numeric type bounded by it.

## Example
```rust
use boundnum::{expr::*, typenum::consts::*, Bounded, Boundable};

fn main() {
    let less4 = Bounded::<u8, Le<Arg, U4>>::new::<U1>();

    let mul_of_two: Bounded<u8, Eq<U0, Rem<Arg, U2>>> =
        (*less4 + 3).bound().unwrap_or(Bounded::new::<U0>());
}
```
