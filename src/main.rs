mod horseshoe;

use horseshoe::Horseshoe;

fn main() {
    let mut server = Horseshoe::new();

    server.get("/whats/up", || println!("hi!"));
    server.get("/whats/up/dog", || println!("What's updog?"));
    server.post("/new", || println!("i post this"));

    server.listen();
}
