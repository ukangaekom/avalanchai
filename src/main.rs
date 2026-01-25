// Defining Modules
mod response;
mod agents;
mod services;
mod tools;
mod connection;

use response::request;
use axum::{
    routing::post,
    Router
};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};





#[tokio::main]
async fn main(){
    dotenv_flow::dotenv_flow().ok();

    // Define CORS layer
    /*
    This endpoint is free for now as it's in demo and testing phase
    */
    let cors = CorsLayer::new()
        .allow_origin(Any) // 👈 Allow requests from any domain
        .allow_methods(Any)
        .allow_headers(Any);

    // Axum router 
    let router_Avalanche_orcinus: Router = Router::new()
        .route("/avax_gpt_agent",post(request))
        .layer(cors); 


    // Define Ip and Port
    let address: &'static str = "0.0.0.0:8080";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();


    println!("Listener on {address}\n");

        
    axum::serve(listener, router_Avalanche_orcinus).await.unwrap();
}


