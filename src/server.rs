use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn ping_health(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello world!".into()))
}

pub async fn start_server() {
    println!("Starting server..");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(ping_health)) });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
