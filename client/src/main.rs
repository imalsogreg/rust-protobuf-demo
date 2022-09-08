use api::my_messages::{Greeting, GreetingRequest};
use api::service::my_service_client::MyServiceClient;

#[tokio::main]
async fn main() {
    let request = GreetingRequest {
        name: env!("USER").to_string(),
    };

    let mut client = MyServiceClient::connect("http://localhost:9000")
        .await
        .expect("Assume the server is running.");

    match client.greet(request).await {
        Ok(resp) => {
            let Greeting {
                message,
                create_time,
                ..
            } = resp.into_inner();
            println!("Response created at {create_time}: {message}");
        }
        Err(status) => {
            println!("Client error: {status:?}");
        }
    }
}
