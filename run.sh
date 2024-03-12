export RUST_LOG=info

# cargo build --release
# ./target/release/rust_todo_axum_mongo

cargo watch -x "run --release"
# cargo watch -x run