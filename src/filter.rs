use crate::{github::fetch_user_details, models::GithubIssue};
use chrono::{DateTime, Utc, Duration};
use reqwest::Error;
use std::collections::HashSet;

// Enhanced spam detection based on specific patterns
pub async fn is_spam_issue(issue: &GithubIssue) -> Result<bool, Error> {
    let spam_indicators = vec![
        "Name of the project",
        "Project type",
        "Site of the project",
        "Github repository of the project",
        "Networks",
        // Add more patterns as observed in spam issues
    ];

    let body_lowercase = issue.body.as_ref().unwrap_or(&"".to_string()).to_lowercase();
    let matches_spam_pattern = spam_indicators.iter().any(|&indicator| body_lowercase.contains(&indicator.to_lowercase()));

    // Assuming `fetch_user_details` is an async function that fetches the user's details, including account creation date
    if matches_spam_pattern {
        // If the body matches known spam patterns, consider further checks or return true immediately
        return Ok(true);
    }

    // Additional check: account age of the issue creator
    if let Ok(user) = fetch_user_details(&issue.user.login).await {
        if let Ok(creation_date) = DateTime::parse_from_rfc3339(&user.created_at) {
            let creation_date_utc = creation_date.with_timezone(&Utc); // Convert to Utc
            if Utc::now() - creation_date_utc < Duration::days(30) { // Accounts younger than 30 days
                return Ok(true);
            }
        }
    }
    

    Ok(false) // If none of the conditions met, it's not detected as spam
}
