mod schema;
mod server;

extern crate log;
extern crate pretty_env_logger;

//We must use this annotation to make our fn main a async function to run forever as long as we dont turn off the api
#[tokio::main]
async fn main() {
    //Start logging
    pretty_env_logger::init();

    //Start the API server to handle requests
    //We must pass an IP address and a port
    server::start(([127, 0, 0, 1], 3000)).await;
}
