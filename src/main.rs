mod horseshoe;

use horseshoe::{Options, Horseshoe};

fn main() {
    let options = Options {
        address: "127.0.0.1".to_string(),
        port: 8080,
    };
    let mut server = Horseshoe::new();

    server.get("/product/:id", |req, res, _cont| {
        let id = &req.params["id"];

        res.send(format!("this is the product data for {}", id).as_str()).unwrap();
    });

    server.listen(options);
}