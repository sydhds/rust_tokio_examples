use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
// use tokio::net::unix::SocketAddr;

/*
 * run server:
 * cargo run --example 01_server_basic
 *
 * client:
 * curl http://127.0.0.1:8000
 */


async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Handling a connection...");
    Ok(Response::new("Hello, World".into()))
}

async fn app_main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    // For every connection, make a Service to handle all incoming HTTP requests on said connection
    let make_svc = make_service_fn(|_conn| async {

        // 'service_fn' is a helper to convert a function that return a Response into a 'Service'
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(app_main());
}

