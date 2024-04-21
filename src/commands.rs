use std::io::{stdout, Write};

use crossterm::{
    execute, queue,
    style::{Print, Stylize},
};

use crate::{api::Api, config};

pub async fn run_login(api_key: String) -> anyhow::Result<()> {
    let result = config::save_api_key(&api_key);

    let path = config::get_directory_path()?;

    match result {
        Ok(_) => println!("API key saved successfully to {:?}", path),
        Err(e) => eprintln!("Error saving API key: {}", e),
    }

    Ok(())
}

pub async fn run_info(api: &Api) -> anyhow::Result<()> {
    let me = api.get_me().await?;
    let mut stdout = stdout();
    queue!(stdout, Print("User is logged in as:".yellow()), Print("\n"))?;
    queue!(
        stdout,
        Print("Email: "),
        Print(me.email.white()),
        Print("\n")
    )?;
    queue!(
        stdout,
        Print("Full Name: "),
        Print(me.fullname.white()),
        Print("\n")
    )?;

    stdout.flush()?;

    Ok(())
}

pub async fn run_current(api: &Api) -> anyhow::Result<()> {
    let current = api.get_current().await?;
    let mut stdout = stdout();

    match current {
        Some(task) => {
            queue!(stdout, Print(task))?;
        }
        None => {
            queue!(stdout, Print("No current task.".yellow()), Print("\n"))?;
        }
    }

    stdout.flush()?;

    Ok(())
}

pub async fn run_todays(api: &Api) -> anyhow::Result<()> {
    let tasks = api.get_today().await?;
    let mut stdout = stdout();

    for task in tasks {
        queue!(stdout, Print(task))?;
    }

    stdout.flush()?;

    Ok(())
}

pub async fn run_restart_latest(api: &Api) -> anyhow::Result<()> {
    api.restart_latest().await?;
    let mut stdout = stdout();

    execute!(stdout, Print("----".yellow()), Print("\n"))?;

    stdout.flush()?;

    Ok(())
}
