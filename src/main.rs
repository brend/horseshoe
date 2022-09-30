mod horseshoe;

use horseshoe::{Horseshoe, Options};

fn main() {
    let options = Options { address: "127.0.0.1".to_string(), port: 7878 };
    let mut server = Horseshoe::new();

    server.get("/whats/up", my_get_handler);

    server.listen(options);
}

use crate::horseshoe::router::{Request, Response};

fn my_get_handler(_req: &mut Request, res: &mut Response) {
    println!("sending a response...");
    let status_line = "HTTP/1.1 200 OK";
    let content = "<html><head><title>This is Horseshoe!</title></head><body><h1>Welcome to the horseshow!</h1></body></html>";
    let length = content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);

    res.write_all(response.as_bytes());
}