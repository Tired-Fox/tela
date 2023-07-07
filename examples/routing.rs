use bytes::Bytes;
use hyper::{body::Incoming, Request};

extern crate web;
use web::prelude::*;
use web::endpoint::{Server, Router, Method};


#[tokio::main]
async fn main() {
    let routes: Router = routes! {
        ["/": get, post] => message,
        ["/hello": delete] => hello,
    };

    println!("{:?}", routes);

    // let router = Router::from([
    //     (vec![Method::Get], "/hello", hello)
    // ]);

    // println!("{:?}", _routes);
    Server::new(([127, 0, 0, 1], 3000)).router(routes).serve().await;
}

fn message(req: Request<Incoming>) ->Result<Bytes, (u16, String)> {
    Ok("Hello, World!".into())
}

// #[get("/hello")]
fn hello(req: Request<Incoming>) -> Result<Bytes, (u16, String)> {
    // Ok("".into())
    // Ok(4.into())
    // Err(404, "")
    // Ok("".into())
    Err((404, "Not Found".to_string()))
}
