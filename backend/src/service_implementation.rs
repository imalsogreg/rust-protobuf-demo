use api::my_messages::{Greeting, GreetingRequest};
use api::service::my_service_server::MyService;
use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};

pub struct MyImplementation;

#[async_trait]
impl MyService for MyImplementation {
    async fn greet(&self, req: Request<GreetingRequest>) -> Result<Response<Greeting>, Status> {
        let GreetingRequest { name } = req.into_inner();
        let create_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid time")
            .as_secs() as u32;

        let greeting = Greeting {
            name: name.clone(),
            message: format!("Greetings, {name}. Great to see you using rust and grpc!"),
            create_time,
        };

        Ok(Response::new(greeting))
    }
}
