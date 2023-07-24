[![Github actions](https://github.com/akoidan/rust-tricks/workflows/Test/badge.svg)](https://github.com/akoidan/rust-tricks/actions)
## Spreadsheets parser

This repo parses [this](https://github.com/stakingrewards/engineering-challenge/blob/backend/transactions.csv) file
Please take a look at the [task](https://github.com/stakingrewards/engineering-challenge/tree/backend)

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

# TODO

This code provides an MVP (Minimum Viable Product) only for the demo purposes. 
It was tested only on [transactions-sample.csv](https://github.com/stakingrewards/engineering-challenge/blob/backend/transactions-sample.csv). This file was corrected on line 17, since it's missing an end bracket.
If further support is needed, mainter should be contacted directly.
