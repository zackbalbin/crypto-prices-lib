# Crypto Prices Lib

### Docs

Crate pubshlied [here](https://crates.io/crates/crypto-prices-lib)
Docs are available [here](https://docs.rs/crypto-prices-lib)

### Example

A example for getting the price of Ergo against Bitcoin

```rust
let price: Option<f64> = get_price("ergo", "bitcoin");
println!("{:?}", price);
```

The result

```
Some(0.00009592777203047117)
```