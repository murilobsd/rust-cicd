use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    run().await?;

    Ok(())
}

async fn run() -> color_eyre::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
