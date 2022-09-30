use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use regex::Regex;

pub mod router;

use router::{Router, Request, Response};

pub struct Horseshoe {
    pub router: Router,
}

impl Horseshoe {
    pub fn new() -> Horseshoe {
        Horseshoe {
            router: Router::new(),
        }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
    
        // println!("Request: {:#?}", http_request);
    
        // GET /whats HTTP/1.1
        let re = Regex::new(r"([A-Z]+) ([^ ]+) HTTP/1\.1").unwrap();
        let mut request = Request {};
        let mut response = Response { stream };
        
        for cap in re.captures_iter(&http_request[0]) {
            let method = &cap[1];
            let path = &cap[2];

            self.router.handle(method, path, &mut request, &mut response);
        }
    }
    
}

impl Horseshoe {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where F: Fn(&mut Request, &mut Response) + 'static + for<'r, 's> Fn(&'r mut Request, &'s mut Response) -> ()
    {
        self.router.add(&"GET", path, handler);
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where F: Fn(&mut Request, &mut Response) + 'static + for<'r, 's> Fn(&'r mut Request, &'s mut Response) -> ()
    {
        self.router.add(&"POST", path, handler);
    }
}