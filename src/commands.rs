use std::io::{stdout, Write};

use crossterm::{
    queue,
    style::{Print, Stylize},
};

use crate::{api::Api, config};

pub async fn store_key(api_key: String) -> anyhow::Result<()> {
    let result = config::save_api_key(&api_key);

    let path = config::get_directory_path()?;

    match result {
        Ok(_) => println!("✅ API key saved successfully to {:?}", path),
        Err(e) => eprintln!("⛔ Error saving API key: {}", e),
    }

    Ok(())
}

pub async fn run_login(username: &str, password: &str) -> anyhow::Result<()> {
    let api_key = Api::get_api_key(username, password).await?;

    let result = config::save_api_key(&api_key);
    let path = config::get_directory_path()?;

    match result {
        Ok(_) => println!("✅ API key saved successfully to {:?}", path),
        Err(e) => eprintln!("⛔ Error saving API key: {}", e),
    }

    Ok(())
}

pub async fn run_stop(api: &Api) -> anyhow::Result<()> {
    let entry = api.get_current().await?;
    let mut stdout = stdout();

    match entry {
        Some(entry) => {
            queue!(stdout, Print("⏳ Found running task ... stopping task\n"),)?;
            api.stop_entry(entry.workspace_id, entry.id).await?;
            queue!(stdout, Print("✅ Task stopped successfully\n"))?;
        }
        None => {
            queue!(stdout, Print("No current task.".yellow()), Print("\n"))?;
        }
    }

    stdout.flush()?;

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

pub async fn run_mine(api: &Api) -> anyhow::Result<()> {
    let tasks = api.get_mine().await?;
    let mut stdout = stdout();

    for task in tasks {
        queue!(stdout, Print(task))?;
    }

    stdout.flush()?;

    Ok(())
}

pub async fn run_last(api: &Api) -> anyhow::Result<()> {
    let tasks = api.get_mine().await?;
    let first = tasks.first();

    let mut stdout = stdout();

    if let Some(entry) = first {
        queue!(stdout, Print(entry))?;
    }

    stdout.flush()?;

    Ok(())
}

pub async fn run_restart(api: &Api) -> anyhow::Result<()> {
    let tasks = api.get_mine().await?;
    let first = tasks.first();

    if first.is_none() {
        return Err(anyhow::Error::msg("No tasks found"));
    }

    let task = api.start_entry(first.unwrap()).await?;

    let mut stdout = stdout();

    queue!(stdout, Print(task))?;

    stdout.flush()?;

    Ok(())
}
