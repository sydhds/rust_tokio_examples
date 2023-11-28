use std::error::Error;
use std::fmt;

use anyhow::Context;
use anyhow::Result as AnyHowResult;

// MyError: manual implementation

#[derive(Debug)]
enum MyError {
    IO(std::io::Error),
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::IO(e) => {
                write!(f, "i/o error: {}", e)
            }
            MyError::Utf8(e) => {
                write!(f, "utf-8 error: {}", e)
            }
        }
    }
}

impl std::error::Error for MyError {}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

// End MyError

// MyError2: same as MyError but using thiserror crate

#[derive(Debug, thiserror::Error)]
enum MyError2 {
    #[error("i/o error: {0}")]
    IO(#[from] std::io::Error),
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

fn read_issue() -> Result<String, MyError> {
    let buf = std::fs::read("/etc/issue")?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

fn read_issue_2() -> Result<String, MyError2> {
    let buf = std::fs::read("/etc/issue")?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

fn read_issue_ko() -> Result<String, MyError2> {
    let buf = std::fs::read("/etc/issue_FOOBAR")?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

fn read_issue_ko_2_no_ctx() -> AnyHowResult<String> {
    let fp = "/etc/issue_FOOBARBAZ";
    let buf = std::fs::read(fp)?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

fn read_issue_ko_2_wt_ctx() -> AnyHowResult<String> {
    let fp = "/etc/issue_FOOBARBAZ";
    let buf = std::fs::read(fp).with_context(|| format!("Failed to read file: {}", fp))?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

type AFnError = Box<dyn Error + Send + Sync>;

async fn read_issue_ko_async() -> Result<String, AFnError> {
    let coro = tokio::spawn(read_issue_ko_async_());
    let res = tokio::try_join!(coro);
    match res {
        Ok((res2,)) => res2,
        Err(e) => {
            // JoinError
            Err(e.into())
        }
    }
}

async fn read_issue_ko_async_() -> Result<String, AFnError> {
    let fp = "/etc/issue_FOOBARBAZ";
    let buf = tokio::fs::read(fp).await?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

async fn read_issue_ko_async_anyhow() -> AnyHowResult<String> {
    let coro = tokio::spawn(read_issue_ko_async_anyhow_());
    let res = tokio::try_join!(coro);
    match res {
        Ok((res2,)) => res2,
        Err(e) => {
            // JoinError
            Err(anyhow::Error::new(e)).with_context(|| "foo")
        }
    }
}

async fn read_issue_ko_async_anyhow_() -> AnyHowResult<String> {
    let fp = "/etc/issue_FOOBARBAZ";
    let buf = tokio::fs::read(fp)
        .await
        .with_context(|| format!("Failed to read file: {}", fp))?;
    let s = String::from_utf8(buf)?;
    Ok(s)
}

fn main() {
    println!("Hello error!");
    println!("{}", read_issue().unwrap());
    println!("{}", read_issue_2().unwrap());
    if let Err(e) = read_issue_ko() {
        println!("Error: {}", e);
        println!("Source: {:?}", e.source());
        // println!("Desc: {}", e.description()); // deprecated
        // println!("Cause: {:?}", e.cause()); // deprecated
        //println!("Backtrace: {:?}", e.backtrace());
    }

    println!("Reading non existing file (WITHOUT CONTEXT):");
    if let Err(e) = read_issue_ko_2_no_ctx() {
        println!("Error: {}", e);
    }
    println!("Reading non existing file (WITH CONTEXT):");
    if let Err(e) = read_issue_ko_2_wt_ctx() {
        println!("Error: {}", e);
    }

    // tokio + anyhow ?
    println!("Reading non existing file async way:");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(read_issue_ko_async());
    println!("res: {:?}", res);

    println!("Reading non existing file async way + anyhow:");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(read_issue_ko_async_anyhow());
    println!("res: {:?}", res);
}
