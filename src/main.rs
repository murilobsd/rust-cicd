use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    println!("Hello, world!");

    Ok(())
}
