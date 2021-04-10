use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status};

mod hello_world {
    include!(concat!(env!("OUT_DIR"), "\\helloworld.rs"));
}

#[derive(Debug, Default)]
pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}\r\n", request);

        let reply = HelloResponse{
            message: format!("Hello {}!", request.into_inner().name).into()
        };

        Ok(Response::new(reply))
    }
}

impl MyGreeter {
    pub fn rtn_service() -> GreeterServer<Self> {
        GreeterServer::new(Self::default())
    }
}
