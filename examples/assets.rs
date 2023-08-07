extern crate wayfinder;

use wayfinder::{prelude::*, Server};

// Run `cargo run --example assets`
// Serve static assets from a given asset path.
#[wayfinder::main]
async fn main() {
    // This gives access to all files from the `files` example
    Server::new().assets("examples/assets/").serve(3000).await
}
