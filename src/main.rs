mod action;
mod api;
mod route;
mod homo;
mod repository;
mod validation;

use std::{collections::HashMap, env::vars, net::SocketAddr, process::exit, sync::Arc};

use dotenv::dotenv;
use log::{error, info};
use tokio::spawn;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    let envs: HashMap<_, _> = vars().collect();

    // コネクション
    let db_config = envs.get("DATABASE_CONFIG").unwrap_or_else(|| {
        error!("Environment variable `DATABASE_CONFIG` must be set!");
        exit(1);
    });

    let (client, connection) = tokio_postgres::connect(db_config, NoTls)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to establish connection to database: {}", e);
            exit(1);
        });
    spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error: {}", e);
            exit(1);
        }
    });
    let client = Arc::new(client);

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

    let routes = route::homochecker(client);

    info!("Listening on {}", listen_address);
    warp::serve(routes).run(listen_address).await;
    Ok(())
}
