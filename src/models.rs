// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubIssue {
    pub id: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: GithubUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestInfo>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequestInfo {
    pub html_url: String,
    // Include other relevant fields as necessary
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    #[serde(rename = "created_at")]
    pub created_at: String, // The creation date in ISO 8601 format, e.g., "2020-05-20T15:09:03Z"

}    