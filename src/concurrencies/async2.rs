use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub async fn run() {
    match case1().await {
        Ok(()) => println!("ok"),
        Err(e) => println!("new error is {}", e),
    }
}

async fn case1() -> Result<(), Report> {
    setup()?;

    info!("case1");

    let client = Client::new();

    fetch_thing(&client, "https://fasterthanli.me").await?;

    Ok(())
}

async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
