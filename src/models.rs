// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubIssue {
    pub id: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: GithubUser,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    #[serde(rename = "created_at")]
    pub created_at: String, // The creation date in ISO 8601 format, e.g., "2020-05-20T15:09:03Z"

}    
