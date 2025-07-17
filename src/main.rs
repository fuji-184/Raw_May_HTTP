mod request;
mod response;
mod server;

use request::{BodyReader, Request};
use response::Response;
use server::start;

fn main() {
    start("0.0.0.0:8080").unwrap().join().unwrap();
}
