use actix_web::{dev::ServerHandle, middleware, rt, web, App, HttpRequest, HttpServer};
use std::{sync::mpsc, thread, time};

async fn index(req: HttpRequest) -> &'static str {
    println!("{}", "REQ: {req:?}");
    "Hello world!"
}

async fn run_app(sender: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:9080");

    // srv is server controller type, `dev::Server`
    let server = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 9080))?
    .workers(2)
    .run();

    // Send server handle back to the main thread
    let _ = sender.send(server.handle());

    server.await
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    println!("spawning thread for server");
    thread::spawn(move || {
        let server_future = run_app(sender);
        rt::System::new().block_on(server_future)
    });

    let server_handle = receiver.recv().unwrap();

    println!("waiting 10 seconds");
    thread::sleep(time::Duration::from_secs(10));

    // Send a stop signal to the server, waiting for it to exit gracefully
    println!("stopping server");
    rt::System::new().block_on(server_handle.stop(true));
}
