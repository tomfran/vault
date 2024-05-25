server:
    clear && cargo run --bin server

client:
    clear && cargo run --bin client

random:
    clear && cargo run --bin random_client

rserver:
    clear && cargo run --release --bin server

rclient:
    clear && cargo run --release --bin client