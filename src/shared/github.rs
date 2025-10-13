use reqwest::{Client, Response};

pub struct GitHubClient<'a> {
    client: &'a Client,
    token: &'a str,
    user_agent: &'a str,
}

impl<'a> GitHubClient<'a> {
    pub fn new(client: &'a Client, token: &'a str, user_agent: &'a str) -> Self {
        Self {
            client,
            token,
            user_agent,
        }
    }

    pub async fn get(&self, url: &str) -> reqwest::Result<Response> {
        self.client
            .get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", self.user_agent)
            .send()
            .await
    }

    pub async fn post_json<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> reqwest::Result<Response> {
        self.client
            .post(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", self.user_agent)
            .json(body)
            .send()
            .await
    }

    pub async fn put_json<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> reqwest::Result<Response> {
        self.client
            .put(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", self.user_agent)
            .json(body)
            .send()
            .await
    }
}

/// Helper to build a client quickly from common inputs
pub fn build_github_client<'a>(client: &'a Client, token: &'a str) -> GitHubClient<'a> {
    GitHubClient::new(client, token, crate::constants::app::USER_AGENT)
}
