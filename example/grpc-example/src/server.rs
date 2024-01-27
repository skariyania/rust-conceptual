use tonic::{transport::Server, Request, Response, Status};

use health::heartbeat_server::{Heartbeat, HeartbeatServer};

use health::{CheckRequest, CheckResponse};

pub mod health {
    tonic::include_proto!("health");
}

#[derive(Debug, Default)]
pub struct ServerHeartbeat {}

#[tonic::async_trait]
impl Heartbeat for ServerHeartbeat {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply: CheckResponse = health::CheckResponse {
            message: format!("status: 200, requester: {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let health_checker = ServerHeartbeat::default();

    Server::builder()
        .add_service(HeartbeatServer::new(health_checker))
        .serve(addr)
        .await?;

    Ok(())
}
