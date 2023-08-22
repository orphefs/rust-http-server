use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {}

//implementation block (methods go here)
impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr, // since key and value are same,
            //we can use this shorthand notation
        }
    }

    pub fn run(self) {
        //takes ownership of the entire struct.
        // `self` will be deallocated when the run() function exits.
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // unwrap terminates the program if the Result is an error

        'outer: loop {
            // here we are labeling the loop
            // "loop" is same as "while true" (infinite loop)

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(buffer));
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
// GET /user?id=1- HTTP/1.1\r\n

// HEADERS \r\n
// BODY
