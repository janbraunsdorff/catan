use axum::Error;


use tokio::net::TcpListener;


pub mod api;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, api::create_main_rounter()).await.unwrap();

    Ok(())
}


