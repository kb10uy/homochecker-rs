mod action;
mod api;
mod homo;
mod repository;

use std::{collections::HashMap, env::vars, net::SocketAddr, process::exit};

use dotenv::dotenv;
use log::{error, info};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use tokio;
use tokio_postgres::NoTls;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    let envs: HashMap<_, _> = vars().collect();

    // コネクションプール
    let db_config = envs
        .get("DATABASE_CONFIG")
        .unwrap_or_else(|| {
            error!("Environment variable `DATABASE_CONFIG` must be set!");
            exit(1);
        })
        .parse()
        .unwrap_or_else(|e| {
            error!("Failed to parse `DATABASE_CONFIG`: {}", e);
            exit(1);
        });

    let manager = PostgresConnectionManager::new(db_config, NoTls);
    let pool = Pool::new(manager).unwrap_or_else(|e| {
        error!("Failed to create database connection pool: {}", e);
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

    let routes = warp::any().and_then(action::check);

    info!("Listening on {}", listen_address);
    warp::serve(routes).run(listen_address).await;
    Ok(())
}
