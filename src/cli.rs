use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "toggl-tui", version, about = "A TUI for Toggl")]
pub struct TogglArgs {
    /// Key for Toggl API
    #[arg(long, short = 'k', value_parser)]
    pub api_key: Option<String>,

    #[clap(subcommand)]
    command: Option<TogglCommand>,
}

#[derive(Debug, Parser, Clone)]
pub enum TogglCommand {
    /// Auth subcommands
    Auth {
        #[clap(subcommand)]
        command: AuthCommand,
    },
    /// Get profile information
    Me,
    /// Get my time entries
    Mine,
    /// Get last time entry
    Last,
    /// Show currently running task
    Current,
    /// Show today's tasks
    Today,
    /// Stop the currently running task
    Stop,
    /// Restart last task
    Restart,
    /// Run in interactive mode
    #[clap(name = "interactive", short_flag = 'i')]
    Interactive,
}

#[derive(Debug, Parser, Clone)]
pub enum AuthCommand {
    /// Set the password in the secure store
    Key {
        #[clap(value_parser)]
        /// The password to set. If not specified, the password
        /// is collected interactively from the terminal
        api_key: String,
    },
    Login {
        /// The username to use
        #[clap(value_parser)]
        username: String,
        /// The password to use
        #[clap(value_parser)]
        password: String,
    },
}

impl TogglArgs {
    pub fn command(&self) -> TogglCommand {
        self.command.clone().unwrap_or(TogglCommand::Interactive)
    }
}

pub fn init_args() -> TogglArgs {
    TogglArgs::parse()
}
