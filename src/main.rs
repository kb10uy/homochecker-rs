mod adapter;

use crate::adapter::{Container, Repositories, Services};
use homochecker_rs::api::route::homochecker;
use std::{collections::HashMap, env::vars, net::SocketAddr, process::exit};

use dotenv::dotenv;
use log::{error, info};
use redis::Client;
use tokio::spawn;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    let envs: HashMap<_, _> = vars().collect();

    // PostgreSQL コネクション
    let db_config = envs.get("DATABASE_CONFIG").unwrap_or_else(|| {
        error!("Environment variable `DATABASE_CONFIG` must be set!");
        exit(1);
    });
    let (pg_client, pg_conn) = tokio_postgres::connect(db_config, NoTls)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to establish connection to database: {}", e);
            exit(1);
        });
    spawn(async move {
        if let Err(e) = pg_conn.await {
            error!("Connection error: {}", e);
            exit(1);
        }
    });

    // Redis コネクション
    let redis_config = envs.get("REDIS_CONFIG").unwrap_or_else(|| {
        error!("Environment variable `REDIS_CONFIG` must be set!");
        exit(1);
    });
    let redis_client = Client::open(&redis_config[..]).unwrap_or_else(|e| {
        error!("Redis connection error: {}", e);
        exit(1);
    });
    let redis = redis_client
        .get_async_connection()
        .await
        .unwrap_or_else(|e| {
            error!("Redis connection error: {}", e);
            exit(1);
        });

    // サーバー
    let listen_address: SocketAddr = envs
        .get("LISTEN_ADDRESS")
        .unwrap_or_else(|| {
            error!("Environment variable `LISTEN_ADDRESS` must be set!");
            exit(1);
        })
        .parse()
        .unwrap_or_else(|e| {
            error!("Failed to parse `LISTEN_ADDRESS`: {}", e);
            exit(1);
        });

    let container = Container::new(Repositories::new(pg_client, redis), Services::new());
    let routes = homochecker(container);

    info!("Listening on {}", listen_address);
    warp::serve(routes).run(listen_address).await;
    Ok(())
}
