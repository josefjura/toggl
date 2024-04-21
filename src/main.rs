use api::URL;

use cli::Command;

use crate::startup::Config;

mod api;
mod cli;
mod commands;
mod config;
mod startup;
mod task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let startup = Config::new()?;

    match startup.command {
        Command::Key { api_key } => {
            commands::run_login(api_key).await?;
        }
        _ => {
            // Handle other cases in a nested match
            let api = api::Api {
                base_url: URL.to_string(),
                api_key: startup
                    .api_key
                    .clone()
                    .ok_or(anyhow::anyhow!("No API key provided"))?,
            };

            match startup.command {
                Command::Me => {
                    commands::run_info(&api).await?;
                }
                Command::Current => {
                    commands::run_current(&api).await?;
                }
                Command::Today => {
                    commands::run_todays(&api).await?;
                }
                Command::RestartLast => {
                    // Restart latest task
                    commands::run_restart_latest(&api).await?;
                }
                Command::Interactive => run_interactive(),
                Command::Key { .. } => unreachable!(), // Already handled above
            }
        }
    }

    Ok(())
}

fn run_interactive() {}
