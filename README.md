## Rust Spreadsheet compiler

A simple compiler in Rust for parsing CSV spreadsheet files with formulas, e.g.
```csv
!date|!transaction_id|!tokens|!token_prices|!total_cost
2022-02-20|=concat("t_", text(incFrom(1)))|btc,eth,dai|38341.88,2643.77,1.0003|=sum(spread(split(D2, ",")))
2022-02-21|=^^|bch,eth,dai|304.38,2621.15,1.0001|=E^+sum(spread(split(D3, ",")))
2022-02-22|=^^|sol,eth,dai|85,2604.17,0.9997|=^^
!fee|!cost_threshold|||
0.09|10000|||
!adjusted_cost||||
=E^v+(E^v*A6)||||
!cost_too_high||||
=text(bte(@adjusted_cost<1>, @cost_threshold<1>)||||
```

## To run this image

#### With Docker

```bash
docker build . -t spreadsheet
docker run -t spreadsheet
```

#### Natively
To run you need to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [Rust](https://doc.rust-lang.org/book/ch00-00-introduction.html). Please execute from the project root.

```bash
cargo run ./assets/transactions.csv
```

You can also run binary with
```bash
cargo build --bin transaction_parser
./target/debug/transaction_parser ./assets/transactions.csv
```
