//Setup Dependencies
use actix_cors::Cors;
use actix_web::{App, web, HttpResponse, HttpServer, Responder, get, post};
use serde::{Deserialize, Serialize};

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
    println!("Filtering issues for repo: {}", info.repository);
    // Placeholder response, replace with actual filtering logic
    HttpResponse::Ok().json(vec![Issue { title: "Example Issue".into() }]) //todo! Come back after working on github API, filter
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
