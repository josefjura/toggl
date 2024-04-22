use api::URL;

use cli::{AuthCommand, TogglCommand};

use crate::startup::Config;

mod api;
mod cli;
mod commands;
mod config;
mod entry;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let startup = Config::new()?;

    match startup.command {
        TogglCommand::Auth {
            command: AuthCommand::Key { api_key },
        } => {
            commands::store_key(api_key).await?;
        }
        TogglCommand::Auth {
            command: AuthCommand::Login { username, password },
        } => {
            commands::run_login(&username, &password).await?;
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
                TogglCommand::Me => {
                    commands::run_info(&api).await?;
                }
                TogglCommand::Stop => {
                    commands::run_stop(&api).await?;
                }
                TogglCommand::Current => {
                    commands::run_current(&api).await?;
                }
                TogglCommand::Today => {
                    commands::run_todays(&api).await?;
                }
                TogglCommand::Mine => {
                    commands::run_mine(&api).await?;
                }
                TogglCommand::Last => {
                    commands::run_last(&api).await?;
                }
                TogglCommand::Restart => {
                    commands::run_restart(&api).await?;
                }
                TogglCommand::Interactive => run_interactive(),
                TogglCommand::Auth { .. } => unreachable!(), // Already handled above
            }
        }
    }

    Ok(())
}

fn run_interactive() {}
