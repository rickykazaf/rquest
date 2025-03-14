use rquest::{redirect::Policy, Impersonate};

#[tokio::main]
async fn main() -> Result<(), rquest::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    // Build a client to impersonate Safari18
    let client = rquest::Client::builder()
        .impersonate(Impersonate::Safari18)
        .build()?;

    let resp = client
        .get("http://google.com/")
        .redirect(Policy::default())
        .send()
        .await?
        .text()
        .await?;

    println!("{}", resp);

    Ok(())
}
