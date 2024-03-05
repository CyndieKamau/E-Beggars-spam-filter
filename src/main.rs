mod github;
mod models;
mod filter;
use std::convert::identity;

//Setup Dependencies
use actix_cors::Cors;
use actix_web::{App, web, HttpResponse, HttpServer, Responder, get, post};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use futures::future::join_all;
use crate::github::fetch_issues; // Import the fetch_issues function
use crate::filter::is_spam_issue; // Import the is_spam_issue function
use crate::models::GithubIssue;

extern crate dotenv;
use dotenv::dotenv;

//Define struct for the Issue filter request
#[derive(Debug, Serialize, Deserialize)]
struct IssueFilterRequest {
    repository: String,
}


//Define struct for the Issue
#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    title: String,
}

// Handler for the index route, serving the static HTML page
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../static/index.html"))
}

// Handler for filtering GitHub issues
#[post("/filter_issues")]
async fn filter_issues(info: web::Json<IssueFilterRequest>) -> impl Responder {
    let repo = &info.repository;
    match fetch_issues(repo).await {
        Ok(issues_json) => {
            if let JsonValue::Array(issues_array) = issues_json {
                let futures = issues_array.into_iter().filter_map(|issue_value| {
                    serde_json::from_value::<GithubIssue>(issue_value).ok().map(|issue| {
                        let issue_clone = issue.clone();
                        async move {
                            if is_spam_issue(&issue_clone).await.unwrap_or(false) {
                                None
                            } else {
                                Some(Issue { title: issue_clone.title }) // Assuming Issue struct is for simplified response
                            }
                        }
                    })
                });

                let filtered_issues: Vec<Issue> = join_all(futures).await.into_iter().filter_map(identity).collect();

                HttpResponse::Ok().json(filtered_issues)
            } else {
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(index)
            .service(filter_issues)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
