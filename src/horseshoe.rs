use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use regex::Regex;

pub mod router;

pub use router::{Router, Request, Response, Continuation, Params};
use router::{match_route};

pub struct Options {
    pub address: String,
    pub port: u32,
}

pub struct Horseshoe {
    pub router: Router,
}

fn parse_headers(http_request: &Vec<String>) -> Vec<(String, String)> {
    let mut headers = vec![];

    for header in http_request.iter().skip(1).take_while(|line| !(**line).is_empty()) {
        let re = Regex::new(r"\s*([^:]+)\s*:\s*(.+)\s*").unwrap();

        for cap in re.captures_iter(header) {
            headers.push((cap[1].to_string(), cap[2].to_string()));
        }
    }

    headers
}

impl Horseshoe {
    pub fn new() -> Horseshoe {
        Horseshoe {
            router: Router::new(),
        }
    }

    pub fn listen(self, options: Options) {
        let listener = TcpListener::bind(format!("{}:{}", options.address, options.port)).unwrap();
    
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
    
        println!("Request: {:#?}", http_request);

        // parse headers
        let headers = parse_headers(&http_request);
    
        // parse first line, whatever that's called
        let re = Regex::new(r"([A-Z]+) ([^ ]+) HTTP/1\.1").unwrap();
        let mut method = String::from("");
        let mut path = String::from("");

        for cap in re.captures_iter(&http_request[0]) {
            method = cap[1].to_uppercase().to_string();
            path = cap[2].to_string();
            break
        }

        if method.is_empty() || path.is_empty() {
            panic!("request line must contain method and path");
        }

        // prepare request and response
        let mut request = Request {
            method,
            path,
            headers,
            params: Params::new(),
        };
        let mut response = Response::new(stream);

        self.router.handle(&mut request, &mut response);
    }
    
}

impl Horseshoe {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where F: Fn(&mut Request, &mut Response, &mut Continuation) + 'static + for<'r, 's, 'c> Fn(&'r mut Request, &'s mut Response, &'c mut Continuation) -> ()
    {
        self.router.add(&"GET", path, handler);
    }

    // pub fn post<F>(&mut self, path: &str, handler: F)
    // where F: Fn(&mut Request, &mut Response) + 'static + for<'r, 's> Fn(&'r mut Request, &'s mut Response) -> ()
    // {
    //     self.router.add(&"POST", path, handler);
    // }
}