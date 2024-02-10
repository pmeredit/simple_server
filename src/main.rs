use open;
use reqwest;
use std::net::TcpListener;
use tide::Request;

#[tokio::main]
async fn main() {
    open::that("http://127.0.0.1:8000").unwrap();
    let mut app = tide::new();
    app.at("/").get(web_action);
    app.listen("127.0.0.1:8000").await.unwrap();
    println!("{}", listener("127.0.0.1", 7999).await);
}

async fn listener(host: &str, port: u16) -> String {
    let urn = format!("{host}:{port}");
    let listener = TcpListener::bind(&urn).unwrap();
    println!("Listening on {}", urn);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        dbg!(stream);
    }

    "returning stuff".to_string()
}

async fn web_action(req: Request<()>) -> tide::Result {
    println!("web action: {:?}", req);
    let url = "127.0.0.1:7999";
    reqwest::get(url).await.unwrap();
    Ok(format!("AUTH SUCCEEDED!!!!").into())
}
