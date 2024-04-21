use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "toggl-tui", version, about = "A TUI for Toggl")]
pub struct TogglArgs {
    /// Key for Toggl API
    #[arg(long, short = 'k', value_parser)]
    pub api_key: Option<String>,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser, Clone)]
pub enum Command {
    /// Set the password in the secure store
    Key {
        #[clap(value_parser)]
        /// The password to set. If not specified, the password
        /// is collected interactively from the terminal
        api_key: String,
    },
    /// Get profile information
    Me,
    /// Show currently running task
    Current,
    /// Show today's tasks
    Today,
    /// Run in interactive mode
    #[clap(name = "interactive", short_flag = 'i')]
    Interactive,
    /// Restart the last task
    #[clap(name = "restart")]
    RestartLast,
}

impl TogglArgs {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Interactive)
    }
}

pub fn init_args() -> TogglArgs {
    TogglArgs::parse()
}
