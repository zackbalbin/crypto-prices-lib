# Crypto Prices Lib

A library for getting the price a cryptocurrency agaisnt another using the CoinGecko API.

### Example

A example for getting the price of Ergo against Bitcoin

```rust
let e: f64 = get_price("ergo", "bitcoin");
```

The result

```
0.00009592777203047117
```