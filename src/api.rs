use chrono::{NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entry::Entry;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct MyKey {
    pub api_token: String,
}

impl Api {
    /// Gets profile information for the authenticated user.
    pub async fn get_api_key(username: &str, password: &str) -> Result<String, ApiError> {
        let client = reqwest::Client::new();
        let me = client
            .get(format!("{}/me", URL))
            .basic_auth(username, Some(password))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<MyKey>()
            .await?;

        Ok(me.api_token)
    }

    /// Gets profile information for the authenticated user.
    pub async fn get_me(&self) -> Result<Me, ApiError> {
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
    pub async fn get_current(&self) -> Result<Option<Entry>, ApiError> {
        let client = reqwest::Client::new();
        let current_task = client
            .get(format!("{}/me/time_entries/current", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Option<Entry>>()
            .await?;

        Ok(current_task)
    }

    pub async fn get_mine(&self) -> Result<Vec<Entry>, ApiError> {
        let client = reqwest::Client::new();
        let tasks = client
            .get(format!("{}/me/time_entries", self.base_url))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Vec<Entry>>()
            .await?;

        Ok(tasks)
    }

    pub async fn get_today(&self) -> Result<Vec<Entry>, ApiError> {
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
            .json::<Vec<Entry>>()
            .await?;

        Ok(today_tasks)
    }

    pub async fn stop_entry(&self, workspace_id: i64, time_entry_id: i64) -> Result<(), ApiError> {
        let client = reqwest::Client::new();

        client
            .patch(format!(
                "{}/workspaces/{}/time_entries/{}/stop",
                self.base_url, workspace_id, time_entry_id
            ))
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?;

        Ok(())
    }

    pub async fn start_entry(&self, entry: &Entry) -> Result<Entry, ApiError> {
        let client = reqwest::Client::new();
        let data = EntryData {
            created_with: "toggl-cli".to_string(),
            description: entry.description.clone(),
            tags: vec![],
            billable: entry.billable,
            workspace_id: entry.workspace_id,
            task_id: entry.task_id,
            project_id: entry.project_id,
            duration: -1,
            start: Utc::now().to_rfc3339(),
            stop: None,
        };
        let data = serde_json::to_string(&data)?;
        let created = client
            .post(format!(
                "{}/workspaces/{}/time_entries",
                self.base_url, entry.workspace_id
            ))
            .body(data)
            .basic_auth(self.api_key.clone(), Some("api_token"))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?
            .json::<Entry>()
            .await?;

        Ok(created)
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("API error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryData {
    pub created_with: String,
    pub description: String,
    pub tags: Vec<String>,
    pub billable: bool,
    pub workspace_id: i64,
    pub duration: i64,
    pub start: String,
    pub stop: Option<String>,
    pub task_id: Option<i64>,
    pub project_id: Option<i64>,
}
