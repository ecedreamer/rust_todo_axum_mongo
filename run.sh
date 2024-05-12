export RUST_LOG=info

# cargo build --release
# ./target/release/rust_todo_axum_mongo

export MONGO_URL="localhost:27017"
export MONGO_USERNAME="dni"
export MONGO_PASSWORD="dni@123"

cargo watch -x "run --release"
# cargo watch -x run