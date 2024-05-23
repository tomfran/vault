server:
    clear && cargo run --bin server

client:
    clear && cargo run --bin client

rserver:
    clear && cargo run --release --bin server

rclient:
    clear && cargo run --release --bin client

clean: 
    cargo clean

test: 
    cargo test

rtest: 
    cargo test --release

build:
    cargo build

rbuild: 
    cargo build --release