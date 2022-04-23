use tokio::io::stdout;
use tokio::io::AsyncWriteExt; // AsyncWriteExt trait

use hyper::Client;
use hyper::body::HttpBody as _; // HttpBody trait
use hyper::{Body, Method, Request, Uri};

type AError = Box<dyn std::error::Error + Send + Sync>;

async fn app_main() -> Result<(), AError> {
    // println!("app_main");

    stdout().write_all(b"### Sending HTTP GET...\n").await?;

    let client = Client::new();
    let uri = "http://127.0.0.1:8000".parse()?;

    let mut resp = client.get(uri).await?; // HTTP GET
    // response status, should be: 200 OK
    assert_eq!(resp.status(), 200);
    stdout().write_all(format!("1- Response status: {}\n", resp.status()).as_ref()).await?;

    // get each chunk
    stdout().write_all(b"Response content:\n").await?;
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    stdout().write_all("\n".as_ref()).await?;

    // POST
    stdout().write_all(b"### Sending HTTP POST (to /echo)...\n").await?;

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8000/echo")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let mut resp = client.request(req).await?;

    // println!("2- Response: {}", resp.status());
    stdout().write_all(format!("2- Response status: {}\n", resp.status()).as_ref()).await?;

    stdout().write_all(b"Response content:\n").await?;
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    stdout().write_all("\n".as_ref()).await?;

    Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _res = rt.block_on(app_main());
    // println!("res: {:?}", _res);
}
