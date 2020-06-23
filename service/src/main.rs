extern crate service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    service::launch_service().await
}
