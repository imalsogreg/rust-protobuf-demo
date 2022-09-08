use api::service::my_service_server::{MyService, MyServiceServer};
use backend::service_implementation::MyImplementation;

use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9000".parse().expect("Valid url");
    Server::builder()
        .add_service(MyServiceServer::new(MyImplementation))
        .serve(addr)
        .await
        .expect("Should run forever")
}
