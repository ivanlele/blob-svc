mod router;
mod handlers;
mod assets;
mod data;
mod config;

use crate::config::get_config;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service_config = get_config();

    let pool = PgPool::connect(service_config.db_url.as_str()).await.unwrap();

    assets::migrate(&pool).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], service_config.port));

    let listener = TcpListener::bind(addr).await?;

    let service = move || {
        service_fn(move |req| router::route(req, pool.clone()))
    };

    let new_service = service();

    println!("Start the listening of connections on: {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let new_service = new_service.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()

                .serve_connection(stream, new_service)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
