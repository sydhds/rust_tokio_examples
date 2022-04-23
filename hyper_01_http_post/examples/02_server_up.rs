use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;

/*
 * cargo run --example 02_server_up
 *
 * curl http://127.0.0.1:8000/echo -X POST -d 'hello world'
 * curl http://127.0.0.1:8000/echo/uppercase -X POST -d 'hello world'
 * curl http://127.0.0.1:8000/echo/reversed -X POST -d 'hello world'
 */


async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Handling a connection...");
    // Ok(Response::new("Hello, World".into()))

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POST'ing data to /echo, e.g.: curl http://127.0.0.1:3000/echo -X POST -d 'hello world'");
        },

        (&Method::POST, "/echo") => {
            // just echo back what was send
            *response.body_mut() = req.into_body();
        },

        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk
                        .iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });
            *response.body_mut() = Body::wrap_stream(mapping);
        },

        (&Method::POST, "/echo/reversed") => {
            let full_body: hyper::body::Bytes = hyper::body::to_bytes(req.into_body()).await?;
            println!("full_body: {:?}", full_body);
            // iter() -> iterator over the slice
            // rev() -> (aka std::iter::Rev): reversed iterator
            // cloned() -> (aka std::iter::Cloned) iterator that clone the underlying iterator
            let reversed: Vec<u8> = full_body
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();
            *response.body_mut() = reversed.into();
        },

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }

    };

    Ok(response)

}

async fn app_main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    // For every connection, make a Service to handle all incoming HTTP requests on said connection
    let make_svc = make_service_fn(|_conn| async {
        // 'service_fn' is a helper to convert a function that return a Response into a 'Service'
        Ok::<_, Infallible>(service_fn(echo))
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

