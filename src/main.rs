mod horseshoe;

use horseshoe::{Horseshoe, Options, router::Continuation};
use crate::horseshoe::router::{Request, Response};

fn main() {
    let options = Options { address: "127.0.0.1".to_string(), port: 7878 };
    let mut server = Horseshoe::new();

    server.get("/whats/up", my_404_handler);
    server.get("/whats/up", my_get_handler);

    server.listen(options);
}

fn my_get_handler(_req: &mut Request, res: &mut Response, _cont: &mut Continuation) {
        println!("sending a response...");
    
    res.send("This is a response").unwrap();
}

fn my_404_handler(req: &mut Request, res: &mut Response, cont: &mut Continuation) {
    res.status(500);
    cont.next();
}