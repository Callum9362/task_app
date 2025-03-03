Useful commands for terminal

SQLLite
sqlite3 todos.db (connect from terminal)
tables . (Lists all tables)
.exit 

Migrations
sqlx migrate add create_todos_table (creates a migration)
sqlx migrate run (runs migrations)
sqlx migrate revert (revets migrations)

Cargo
cargo build
cargo test
cargo run

Example payload sent from terminal
curl -X POST http://127.0.0.1:3000/todos \
-H "Content-Type: application/json" \
-d '{"title":"Learn Rust More","completed":false}'

{"id":"282ff6ac-9bde-4b54-80ea-cddb00d55be4","title":"Learn Rust","completed":false}%     