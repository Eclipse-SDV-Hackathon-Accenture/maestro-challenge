use actix_web::{web, App, HttpServer};
use std::net::SocketAddr;

mod resource_statistic_job;
mod routes;

use resource_statistic_job::init_resource_statistic_cache;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let socket_address =
        std::env::var("RESOURCE_STATISTICS_API_SOCKET").unwrap_or("0.0.0.0:8080".to_string());
    let socket_address: SocketAddr = socket_address.parse()?;

    let (item_cache, cache_sweep_handle, cache_sweep_cancel) = init_resource_statistic_cache();
    HttpServer::new(move || {
        App::new()
            .service(routes::statistics)
            .app_data(web::Data::from(item_cache.clone()))
    })
    .workers(2)
    .bind((socket_address.ip(), socket_address.port()))?
    .run()
    .await?;

    cache_sweep_cancel.cancel();
    cache_sweep_handle.await.unwrap();
    Ok(())
}
