use axum::serve;
use tokio::{self, net::TcpListener};

use crate::{routes::app, state::AppState};
mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let app = app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("taskr server listening at: http://localhost:3000/");

    serve(listener, app).await.unwrap();
}
