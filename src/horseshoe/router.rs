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
    
    pub fn write_all(&mut self, buf: &[u8]) {
        self.stream.write_all(&buf).unwrap();
    }

    pub fn send(&mut self, content: &str) {
        let status_line = format!("HTTP/1.1 {} OK", match self.status_code { Some(code) => code, None => 200 });
        let length = content.len();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);

        self.stream.write_all(response.as_bytes()).unwrap();
    }
}

pub struct Callback<F>
where
    F: Fn(&mut Request, &mut Response) -> (),
{
    pub handler: F,
}

pub struct Router {
    routes: HashMap<Method, HashMap<Route, Vec<Callback<Box<dyn Fn(&mut Request, &mut Response) -> ()>>>>>
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
                
                for callback in callbacks {
                    (callback.handler)(request, response);
                }

            } else {
                println!("No \"{}\" route registered for path \"{}\"", &request.method, &request.path);

                println!("{:?}", &routes_for_method.keys());
            }
        } else {
            println!("No routes registered for method \"{}\"", &request.method);
        }
    }

    pub fn add<F>(&mut self, method: &str, path: &str, handler: F)
    where F: Fn(&mut Request, &mut Response) + 'static + for<'r, 's> Fn(&'r mut Request, &'s mut Response) -> ()
    {
        let method = &method.to_uppercase().to_string();
        let callback: Callback<Box<dyn Fn(&mut Request, &mut Response) -> ()>> = Callback { handler: Box::new(handler) };

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