use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

  //  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind TCP listener");
    // let port = listener.local_addr()?.port();
    run(listener)?.await
}


