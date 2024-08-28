use std::error::Error;

use axum::{
    http::StatusCode, response::IntoResponse, serve::Serve, Router,
    routing::{get,post}
};

pub mod domain;
pub mod routes;
pub mod services;


pub struct Application {
    server: Serve<Router, Router>,
    pub address: String
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let app = Router::new()
            .route("/", get(health_check))
            .route("/create-tx", post(routes::create_tx));

        let router = app;

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app_inst = Application {
            server,
            address
        };

        Ok(app_inst)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await  
    }
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
