mod horseshoe;

use horseshoe::Horseshoe;

fn main() {
    let mut server = Horseshoe::new();

    server.get("/whats/up", || println!("hi!"));

    server.listen();
}
