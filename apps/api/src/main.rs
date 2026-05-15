mod modules;
mod shared;

use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::modules::books::BookRouter;
use crate::modules::books::PostgresBookRepository;
use crate::shared::app_state::AppState;
use crate::shared::postgres::create_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pool().await;

    let book_repository = Arc::new(PostgresBookRepository::new(pool));

    let state = Arc::new(AppState::new(book_repository));

    let app = Router::new()
        .nest("/books", BookRouter::new())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
