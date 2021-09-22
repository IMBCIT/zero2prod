use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tcp = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = tcp.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
