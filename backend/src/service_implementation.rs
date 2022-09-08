use async_trait;
use api::service::my_service_server::{MyService};
use api::my_messages::{GreetingRequest, Greeting};

use tonic::{Request, Response, Status};

pub struct MyImplementation {
}

#[async_trait::async_trait]
impl MyService for MyImplementation {
  async fn greet(&self, req: Request<GreetingRequest>) -> Result<Response<Greeting>, Status> {
      unimplemented!()
  }
}
