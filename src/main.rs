mod action;
mod api;
mod homo;
mod repository;

use tokio;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes = warp::any().and_then(action::check);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}
