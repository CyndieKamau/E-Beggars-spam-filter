use crate::models::{GithubIssue, GithubUser};
use reqwest::{Client, Error, header};
use serde_json::Value;
use std::env;

pub async fn fetch_issues(owner_repo: &str) -> Result<Vec<GithubIssue>, Error> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/issues", owner_repo);

    // Retrieve the GitHub token from environment variables
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let response = client.get(url)
        .header(header::USER_AGENT, "YourApp (contact@example.com)") // Change to your app info
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .bearer_auth(github_token) // Use the token for authentication
        .send()
        .await?;

    // Check if the response status is success, handle errors or redirection as needed
    if response.status().is_success() {
        let issues = response.json::<Vec<GithubIssue>>().await?;
        Ok(issues)
    } else {
        // Handle error responses, e.g., by logging or converting to a custom error type
        Err(Error::from(response.error_for_status().unwrap_err()))
    }
}

//Fetch a user's details
pub async fn fetch_user_details(username: &str) -> Result<GithubUser, Error> {
    let client = Client::new();
    let url = format!("https://api.github.com/users/{}", username);

    // Retrieve the GitHub token from environment variables, same as above
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let response = client.get(url)
        .header(header::USER_AGENT, "YourApp (contact@example.com)")
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .bearer_auth(github_token) // Authenticate the request
        .send()
        .await?;

    if response.status().is_success() {
        let user = response.json::<GithubUser>().await?;
        Ok(user)
    } else {
        Err(Error::from(response.error_for_status().unwrap_err()))
    }
}
