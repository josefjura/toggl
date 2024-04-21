use chrono::{NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use crate::task::Task;

pub const URL: &str = "https://api.track.toggl.com/api/v9";

pub struct Api {
    pub base_url: String,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    pub email: String,
    pub fullname: String,
}

impl Api {
    /// Gets profile information for the authenticated user.
    pub async fn get_me(&self) -> Result<Me, reqwest::Error> {
        let client = reqwest::Client::new();
        let me = client
            .get(format!("{}/me", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Me>()
            .await?;

        Ok(me)
    }

    // null
    // {"id":3415527565,"workspace_id":8034151,"project_id":202151475,"task_id":null,"billable":false,"start":"2024-04-21T10:35:28+00:00","stop":null,"duration":-1713695728,"description":"Test Item","tags":[],"tag_ids":[],"duronly":true,"at":"2024-04-21T10:35:29+00:00","server_deleted_at":null,"user_id":10324100,"uid":10324100,"wid":8034151,"pid":202151475,"permissions":null}
    pub async fn get_current(&self) -> Result<Option<Task>, reqwest::Error> {
        let client = reqwest::Client::new();
        let current_task = client
            .get(format!("{}/me/time_entries/current", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Option<Task>>()
            .await?;

        Ok(current_task)
    }

    pub async fn get_today(&self) -> Result<Vec<Task>, reqwest::Error> {
        let client = reqwest::Client::new();
        let today_date = Utc::now().with_time(NaiveTime::MIN);
        let today = today_date.single().unwrap();
        let today_tasks = client
            .get(format!("{}/me/time_entries", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .query(&[("since", today.timestamp())])
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;

        Ok(today_tasks)
    }

    pub async fn restart_latest(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let current_task = client
            .get(format!("{}/me/tasks", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .query(&[("include_not_active", true)])
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?;
        // .json::<Option<Task>>()
        // .await?;

        println!("{:?}", current_task.text().await?);

        Ok(())
    }
}
