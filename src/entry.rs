use std::{
    fmt::{self, Display},
    io::{self},
};

use chrono::{DateTime, Utc};
use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: i64,
    pub workspace_id: i64,
    pub description: String,
    pub billable: bool,
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
    pub task_id: Option<i64>,
    pub project_id: Option<i64>,
}

impl Display for Entry {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = io::stdout();

        let task_active = self.stop.is_none();
        let status_symbol = if task_active { '✅' } else { '❌' };
        let status_color = if task_active {
            Color::Green
        } else {
            Color::Red
        };

        queue!(
            stdout,
            SetForegroundColor(status_color),
            Print(format!("Active: {}\n", status_symbol))
        )
        .map_err(|_| fmt::Error)?;
        queue!(
            stdout,
            SetForegroundColor(Color::Grey),
            Print("Description: "),
            SetForegroundColor(Color::White),
            Print(format!("{}\n", self.description))
        )
        .map_err(|_| fmt::Error)?;
        queue!(
            stdout,
            SetForegroundColor(Color::Grey),
            Print("Start: "),
            SetForegroundColor(Color::White),
            Print(format!("{}\n", output_date(self.start)))
        )
        .map_err(|_| fmt::Error)?;

        if let Some(stop) = &self.stop {
            queue!(
                stdout,
                SetForegroundColor(Color::Grey),
                Print("Stop: "),
                SetForegroundColor(Color::White),
                Print(format!("{} UTC\n", output_date(*stop)))
            )
            .map_err(|_| fmt::Error)?;
        }

        queue!(stdout, Print("\n"), ResetColor).map_err(|_| fmt::Error)
    }
}

// pub fn print_task(task: &Task) -> anyhow::Result<()> {
//     let mut stdout = stdout();

//     // Determine if the task is active based on whether the stop time is None
//     let task_active = task.stop.is_none();
//     let status_symbol = if task_active { '✅' } else { '❌' };
//     let status_color = if task_active {
//         Color::Green
//     } else {
//         Color::Red
//     };

//     // Simple formatting with basic color enhancement
//     queue!(
//         stdout,
//         SetForegroundColor(status_color),
//         Print(format!("Active: {}\n", status_symbol))
//     )?;
//     queue!(
//         stdout,
//         SetForegroundColor(Color::Grey),
//         Print("Description: "),
//         SetForegroundColor(Color::White),
//         Print(format!("{}\n", task.description))
//     )?;
//     queue!(
//         stdout,
//         SetForegroundColor(Color::Grey),
//         Print("Start: "),
//         SetForegroundColor(Color::White),
//         Print(format!("{}\n", output_date(task.start)))
//     )?;

//     if let Some(stop) = &task.stop {
//         queue!(
//             stdout,
//             SetForegroundColor(Color::Grey),
//             Print("Stop: "),
//             SetForegroundColor(Color::White),
//             Print(format!("{} UTC\n", output_date(*stop)))
//         )?;
//     }

//     queue!(stdout, Print("\n"), ResetColor)?;
//     Ok(())
// }

fn output_date(date: chrono::DateTime<Utc>) -> String {
    date.format("%d.%m.%Y %H:%M:%S").to_string()
}
