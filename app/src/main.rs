use axum::Error;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing_subscriber::fmt()
        .with_target(true)
        .log_internal_errors(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    axum::serve(listener, api::create_main_rounter())
        .await
        .unwrap();

    Ok(())
}
