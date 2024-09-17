use anyhow::{Context, Result};
use reqwest::{header, Client};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
}

pub struct GitHubAPI {
    client: Client,
}

impl GitHubAPI {
    pub fn new() -> Result<Self> {
        let client = Self::build_client()?;
        Ok(Self { client })
    }

    pub async fn get_all_releases(&self) -> Result<Vec<Release>> {
        let mut all_releases = Vec::new();
        let mut page = 1;
        let per_page = 100; // GitHub's maximum allowed value

        loop {
            let url = format!(
                "https://api.github.com/repos/buildkite/agent/releases?page={}&per_page={}",
                page, per_page
            );

            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to fetch releases: HTTP {}",
                    response.status()
                ));
            }

            let releases: Vec<Release> = response.json().await?;

            if releases.is_empty() {
                break;
            }

            all_releases.extend(releases);
            page += 1;
        }

        Ok(all_releases)
    }

    fn build_client() -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("bap-cli"),
        );

        if let Ok(token) = env::var("GITHUB_TOKEN") {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("token {}", token))
                    .context("Invalid GitHub token")?,
            );
        }

        Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")
    }
}
