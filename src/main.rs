use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuraton = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
