// use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};

/*
 * cargo run
 *
 * curl http://127.0.0.1:8000/echo -X POST -d 'hello world'
 * curl http://127.0.0.1:8000/echo/uppercase -X POST -d 'hello world'
 * curl http://127.0.0.1:8000/echo/reversed -X POST -d 'hello world'
 *
 * STOP the server
 * curl http://127.0.0.1:8000/stop -X POST -d ''  // will send false to mpsc
 * curl http://127.0.0.1:8000/stop -X POST -d '1' // will send true to mpsc
 */


async fn echo(req: Request<Body>, tx: Sender<bool>) -> Result<Response<Body>, hyper::Error> {
    println!("Handling a connection...");
    // Ok(Response::new("Hello, World".into()))

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => {
            *response.body_mut() = Body::from(
                "Try POST'ing data to /echo, e.g.: curl http://127.0.0.1:3000/echo -X POST -d 'hello world'");
        },

        (&Method::POST, "/stop") => {
            println!("[echo] Got stop...");

            let full_body: hyper::body::Bytes = hyper::body::to_bytes(req.into_body()).await?;

            // tx.send(false).await;
            let to_send: bool = match full_body.is_empty() {
                true => false,
                false => true,
            };

            if let Err(e) = tx.send(to_send).await {
                eprintln!("Unable to send to channel: {}", e);
            }

            // Response::new("Thanks for stopping the server...".into()))
            *response.body_mut() = Body::from("Thanks for that!! Will stop the server...");
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

async fn shutdown_signal() {

    // Wait for Ctrl-C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn wait_for_stop_true(mut rx: Receiver<bool>) {

    loop {
        let stop = rx.recv().await;
        println!("Got a stop value: {:?}", stop);
        match stop {
            Some(stop_) if stop_ == true => break,
            _  => continue,
        }
    }
}

async fn shutdown_from_channel(rx: Receiver<bool>) {

    // as an exercise, use a oneshot channel (triggered by HTTP POST to url: /stop)
    // to stop the server

    // rx.recv().await;
    wait_for_stop_true(rx).await;
}


async fn app_main() {

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    // For every connection, make a Service to handle all incoming HTTP requests on said connection

    /*
    let make_svc = make_service_fn(|_conn| async {

        // 'service_fn' is a helper to convert a function that return a Response into a 'Service'
        Ok::<_, Infallible>(service_fn(echo))
    });
    */

    let (tx, rx) = mpsc::channel(1);

    let make_svc = make_service_fn(move |_conn| {
        // move tx to closure
        // closure can be called multiple times (so we clone)
        let tx = tx.clone();
        async move {
            // async block is only exec once
            // so move tx to the closure
            Ok::<_, hyper::Error>(service_fn(
                move |req| {
                    // this closure is also be called multiple times so make a clone too
                    // and move the clone into the async block
                    let tx = tx.clone();
                    async move { echo(req, tx).await }
                }
            ))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    // let graceful = server.with_graceful_shutdown(shutdown_signal());
    let graceful = server.with_graceful_shutdown(shutdown_from_channel(rx));

    println!("Listening on http://{}", addr);

    if let Err(e) = graceful.await {
        eprintln!("Server error: {}", e);
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(app_main());
}
