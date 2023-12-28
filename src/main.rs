mod router;
#[tokio::main]
async fn main() -> std::io::Result<()>  {
    // start web server
    router::web_server().await
}
