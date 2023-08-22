use std::net::TcpListener;

pub struct Server {
    addr: String,
}

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

        let listener = TcpListener::bind(&self.addr).unwrap();

        'outer: loop {
            // here we are labeling the loop
            // "loop" is same as "while true" (infinite loop)
            break;
        }
    }
}
// GET /user?id=1- HTTP/1.1\r\n

// HEADERS \r\n
// BODY
