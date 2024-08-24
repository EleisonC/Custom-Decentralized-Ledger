# Custom-Decentralized-Ledger
A proof-of-work decentralized ledger from the ground up using Rust


## Setup & Building
```bash
cargo install cargo-watch
cd transaction
cargo build
cd ..
```

## Run transaction server locally (Manually)
#### Transaction service
```bash
cd transaction
cargo watch -q -c -w src/ -x run
```

visit http://localhost:2000