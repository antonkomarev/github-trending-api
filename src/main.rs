mod http;
mod parser;

use std::error::Error;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let address = format!("{}:{}", args.host, args.port);
    let tcp_listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Starting service on {}", address);
    axum::serve(tcp_listener, get_router()).await?;

    Ok(())
}

fn get_router() -> axum::Router {
    axum::Router::new()
        .route(
            "/v1/listTrendingDeveloper",
            axum::routing::post(http::v1::list_trending_developer_controller::invoke),
        )
        .route(
            "/v1/listTrendingRepository",
            axum::routing::post(http::v1::list_trending_repository_controller::invoke),
        )
}
