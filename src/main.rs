use crate::startup::Startup;

mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let startup = Startup::new()?;
    println!("Starting up the server... {}", startup.api_key);
    Ok(())
}
