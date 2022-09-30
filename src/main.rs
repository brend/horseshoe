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
    
    res.status(404).send("This is a response");
}