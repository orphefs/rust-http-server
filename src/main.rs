use http::Request;
use http::Method;
use server::Server;

mod server; // here the compiler copy/pastes the contents of server.rs in here
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
    // dbg!(&string);
    // dbg!(string_slice);
}
