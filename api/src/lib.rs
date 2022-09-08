pub mod my_messages {
    tonic::include_proto!("my_messages");
}

pub mod service {
    tonic::include_proto!("my_service");
}

#[cfg(test)]
mod tests {
    use super::my_messages::{GreetingRequest, Greeting};

    #[test]
    fn it_defines_greeting_requests() {
        let greeting_request = GreetingRequest {
            name: "Ferris".to_string(),
        };
        assert_eq!(greeting_request.name.as_str(), "Ferris");
    }

    #[test]
    fn it_defines_greetings() {
        let greeting = Greeting {
            name: "Ferris".to_string(),
            message: "Hello, crustacean.".to_string(),
            create_time: 1,
        };
        assert_eq!(greeting.create_time, 1);
    }
}
