use crate::models::{GithubIssue, GithubUser};
use reqwest::{Client, Error, header};
use serde_json::Value;
use std::env;

extern crate dotenv;
use dotenv::dotenv;

pub async fn fetch_issues(owner_repo: &str) -> Result<serde_json::Value, Error> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/issues", owner_repo);

    // Retrieve the GitHub token from environment variables
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let response = client.get(url)
        .header(header::USER_AGENT, "YourApp (contact@example.com)") // Change to your app info
        .header(header::ACCEPT, "application/vnd.github+json")
        .bearer_auth(github_token) // Use the token for authentication
        .send()
        .await?;

    // Check if the response status is success, handle errors or redirection as needed
    if response.status().is_success() {
        let raw_json = response.text().await.expect("Failed to get text from response");
        println!("Raw JSON: {}", raw_json);
        // Deserialize raw JSON into serde_json::Value for inspection
        let issues = serde_json::from_str::<serde_json::Value>(&raw_json).expect("Failed to deserialize into serde_json::Value");
        println!("{:#?}", issues);
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
        .header(header::ACCEPT, "application/vnd.github+json")
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use dotenv::dotenv;
    use serde_json::json;

    #[tokio::test]
    async fn test_fetch_user_details() {
        dotenv().ok(); // Load .env for tests, ensuring GITHUB_TOKEN is available

        // Use your GitHub username
        let username = "CyndieKamau";
        let user_result = fetch_user_details(username).await;

        assert!(user_result.is_ok(), "Failed to fetch user details");

        let user = user_result.expect("Failed to unwrap user details");

        // Check if the created_at field is parsed correctly
        assert!(
            !user.created_at.is_empty(),
            "The created_at field should not be empty"
        );

        // Additional checks can be performed here, such as verifying the format of the created_at date string
        println!("User {} was created at {}", user.login, user.created_at);
    }

    
    #[tokio::test]
    async fn test_fetch_issues_two() {
        dotenv().ok(); // Ensure .env variables are loaded
        println!("Using GitHub Token: {:?}", env::var("GITHUB_TOKEN"));
        let repo = "starknet-edu/starknetbook";
        let result = fetch_issues(repo).await;

        assert!(result.is_ok(), "Fetching issues resulted in an error.");

        let issues_value = result.expect("Failed to unwrap issues as serde_json::Value");

        // Check if the returned value is an array of issues
        assert!(issues_value.is_array(), "The issues data should be an array");
        assert!(!issues_value.as_array().unwrap().is_empty(), "The issues list is empty, but was expected to contain at least one issue.");

        println!("Fetched {} issues as serde_json::Value.", issues_value.as_array().unwrap().len());
    }

    #[tokio::test]
    async fn test_deserialize_issue_with_pull_request_info() {
        // Mock JSON data for an issue that is actually a pull request
        let issue_json = json!({
            "id": 216526498,
            "title": "refac: add Starknet-devnet-rs",
            "body": null,
            "user": {
                "login": "estheroche",
                "id": 125284347,
                "created_at": "2020-05-20T15:09:03Z"
            },
            "pull_request": {
                "html_url": "https://github.com/starknet-edu/starknetbook/pull/339"
            }
            // include other fields as necessary
        });

        // Attempt to deserialize the JSON into a GithubIssue
        let issue_result: Result<GithubIssue, _> = serde_json::from_value(issue_json);

        assert!(issue_result.is_ok(), "Failed to deserialize GitHub issue");

        let issue = issue_result.expect("Failed to unwrap issue");

        // Perform some checks to ensure the issue (pull request) is deserialized correctly
        assert_eq!(issue.id, 216526498);
        assert_eq!(issue.title, "refac: add Starknet-devnet-rs");
        assert!(issue.body.is_none(), "Body should be None");

        // Use `if let` to access the pull request info without moving it
        if let Some(pull_request) = &issue.pull_request {
            assert_eq!(pull_request.html_url, "https://github.com/starknet-edu/starknetbook/pull/339");
        } else {
            panic!("Pull request info should be present");
        }

        // Since we're not moving `issue.pull_request`, it's safe to assert on `issue.user` afterwards
        assert_eq!(issue.user.login, "estheroche");

        println!("Deserialized issue (pull request): {:?}", issue);
    }


}
