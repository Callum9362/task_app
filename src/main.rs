use crate::{db::connect, routes::create_routes};

mod models;
mod db;
mod routes;
mod todo_controller;

#[tokio::main]
async fn main() {
    let pool = connect().await;

    let app = create_routes(pool);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
