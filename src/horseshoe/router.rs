use std::{collections::HashMap, net::TcpStream, io::Write};

type Method = String;
type Route = String;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

pub struct Response {
    stream: TcpStream,
    status_code: Option<u32>,
}

fn code_name(status_code: u32) -> String {
    let names: HashMap<u32, &str> = [
        (100, "Continue"),
        (101, "Switching Protocols"),
        (102, "Processing"),
        (103, "Early Hints"),
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non-Authoritative Information"),
        (204, "No Content"),
        (205, "Reset Content"),
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (500, "Internal Server Error"),
    ]
    .iter()
    .cloned().collect();

    match names.get(&status_code) {
        Some(name) => name,
        None => ""
    }.to_string()
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        Response {
            stream,
            status_code: None,
        }
    }

    pub fn status(&mut self, code: u32) -> &mut Self {
        self.status_code = Some(code);

        return self;
    }
    
    pub fn send(&mut self, content: &str) -> Result<(), std::io::Error> {
        let status_code = match self.status_code { Some(code) => code, None => 200 };
        let status_line = format!("HTTP/1.1 {} {}", status_code, code_name(status_code));
        let length = content.len();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);

        self.stream.write_all(response.as_bytes())
    }
}

pub struct Callback<F>
where
    F: Fn(&mut Request, &mut Response, &mut Continuation) -> (),
{
    pub handler: F,
}

pub struct Continuation<'a> {
    //handlers: Vec<fn(&mut Request, &mut Response, &Continuation) -> ()>,
    handlers: &'a Vec<Callback<Box<dyn Fn(&mut Request, &mut Response, &mut Continuation) -> ()>>>,
    next_index: usize,
    request: &'a mut Request,
    response: &'a mut Response,
}

impl<'a> Continuation<'a> {
    pub fn next(&mut self) {
        if self.next_index >= self.handlers.len() {
            return;
        }

        unsafe {
            let p = self as *mut Continuation<'a>;
            let callback = &(*p).handlers[self.next_index];

            (&mut *p).next_index += 1;
            (callback.handler)(self.request, self.response, &mut *p);
        }
    }
}

pub struct Router {
    routes: HashMap<Method, HashMap<Route, Vec<Callback<Box<dyn Fn(&mut Request, &mut Response, &mut Continuation) -> ()>>>>>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn handle(&self, request: &mut Request, response: &mut Response) {
        if let Some(routes_for_method) = self.routes.get(&request.method) {
            if let Some(callbacks) = routes_for_method.get(&request.path) {
                let mut continuation = Continuation {
                    request: request,
                    response: response,
                    next_index: 0,
                    handlers: callbacks,
                };
                
                continuation.next();
            } else {
                println!("No \"{}\" route registered for path \"{}\"", &request.method, &request.path);

                println!("{:?}", &routes_for_method.keys());
            }
        } else {
            println!("No routes registered for method \"{}\"", &request.method);
        }
    }

    pub fn add<F>(&mut self, method: &str, path: &str, handler: F)
    where F: Fn(&mut Request, &mut Response, &mut Continuation) + 'static + for<'r, 's, 'c> Fn(&'r mut Request, &'s mut Response, &'c mut Continuation) -> ()
    {
        let method = &method.to_uppercase().to_string();
        let callback: Callback<Box<dyn Fn(&mut Request, &mut Response, &mut Continuation) -> ()>> = Callback { handler: Box::new(handler) };

        // get routes for method
        if !self.routes.contains_key(method) {
            self.routes.insert(String::from(method), HashMap::new());
        }

        let routes_for_method = self.routes.get_mut(method).unwrap();

        // get handlers for path
        if !routes_for_method.contains_key(&path.to_string()) {
            routes_for_method.insert(path.to_string(), Vec::new());
        }

        let handlers = routes_for_method.get_mut(&path.to_string()).unwrap();

        // add new handler
        handlers.push(callback);
    }
}