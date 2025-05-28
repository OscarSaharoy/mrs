use reqwest::Client;
use serde::Deserialize;
use dotenvy::dotenv;
use std::env;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct MergeRequest {
    iid: u64,
    title: String,
    detailed_merge_status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // update these to get this working for you
    // the .env file should include a line like: GITLAB_TOKEN=...
    let author_username = "oscarsaharoylccc";
    let dotenv_path = "/Users/oscarsaharoy/projects/mrs/.env";

    dotenvy::from_path(Path::new(dotenv_path)).ok();
    let gitlab_url = "https://gitlab.com/api/v4/merge_requests";
    let token = env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN must be set in .env");

    let client = Client::new();
    let response = client
        .get(gitlab_url)
        .query(&[
            ("scope", "all"),
            ("state", "opened"),
            ("author_username", author_username),
            ("per_page", "100"),
        ])
        .header("PRIVATE-TOKEN", token)
        .send()
        .await?
        .json::<Vec<MergeRequest>>()
        .await?;

    println!("You have {} open MRs.", response.len());
    for mr in response {
        println!("!{} [{}] {}", mr.iid, format!("{:15}", mr.detailed_merge_status), mr.title);
    }

    Ok(())
}

