
mod horseshoe {
    use std::net::{TcpListener, TcpStream};
    use std::io::{prelude::*, BufReader};
    use regex::Regex;
    use std::collections::HashMap;

    struct Callback<F>
    where
        F: Fn() -> (),
    {
        pub foo: F,
    }

    impl<F> Callback<F>
    where
        F: Fn() -> (),
    {
        fn new(foo: F) -> Self {
            Self { foo }
        }
    }


    pub struct Horseshoe {
        routes: HashMap<String, Callback<Box<dyn Fn() -> ()>>>
    }

    impl Horseshoe {
        pub fn new() -> Horseshoe {
            Horseshoe {
                routes: HashMap::<String, Callback<Box<dyn Fn() -> ()>>>::new()
            }
        }

        pub fn listen(self) {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        
            for stream in listener.incoming() {
                let stream = stream.unwrap();
        
                self.handle_connection(stream);
            }
        }

        pub fn get<F>(&mut self, path: &str, handler: F)
        where F: Fn() + 'static
        {
            let callback: Callback<Box<dyn Fn() -> ()>> = Callback { foo: Box::new(handler) };

            self.routes.insert(path.to_string(), callback);
        }

        fn handle_connection(&self, mut stream: TcpStream) {
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
        
            println!("Request: {:#?}", http_request);
        
            // GET /whats HTTP/1.1
            let re = Regex::new(r"([A-Z]+) ([^ ]+) HTTP/1\.1").unwrap();
            
            for cap in re.captures_iter(&http_request[0]) {
                let method = &cap[1];
                let path = &cap[2];
        
                println!("method: {}, path: {}", method, path);

                if let Some(callback) = self.routes.get(&path.to_string()) {
                    (callback.foo)();
                } else {
                    println!("Unhandled route");
                }
            }
        }
        
    }
}

use crate::horseshoe::Horseshoe;

fn main() {
    let mut server = Horseshoe::new();

    server.get("/whats/up", || println!("hi!"));

    server.listen();
}
