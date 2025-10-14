use eyre::Result;
use log::{error, info, warn};
use reqwest::Client;
use serde_json::Value;

use crate::app::CloudflareConfig;
use crate::shared::error::GenWebBlogError;

async fn get_existing_domains(client: &Client, config: &CloudflareConfig) -> Result<Vec<String>> {
    let account_id = &config.account_id;
    let project_name = &config.project_name;
    let url = format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/pages/projects/{project_name}/domains");

    let response = client
        .get(&url)
        .header(
            "Authorization",
            format!("Bearer {api}", api = config.api_token),
        )
        .header("User-Agent", "Rust-Deploy-Bot/1.0")
        .send()
        .await
        .map_err(|e| GenWebBlogError::network(format!("Failed to request domains: {e}")))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("│  ❌ Failed to fetch existing domains: {error_text}");
        return Err(GenWebBlogError::cloudflare(format!(
            "Failed to fetch existing domains: {error_text}"
        ))
        .into());
    }

    let response_json: serde_json::Value = response.json().await.map_err(GenWebBlogError::Http)?;

    let existing_domains = response_json["result"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|domain| domain["name"].as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    Ok(existing_domains)
}

pub async fn add_domains_to_cloudflare(
    client: &Client,
    config: &CloudflareConfig,
    domain: &String,
) -> Result<()> {
    let existing_domains = get_existing_domains(client, config).await?;

    if domain.starts_with("http://") || domain.starts_with("https://") {
        warn!("⚠️ Invalid domain format: '{domain}' (Remove 'http://' or 'https://')");
        return Ok(());
    }

    if is_domain_already_added(client, config, domain).await? {
        info!("│  ⚠️ Domain '{domain}' already present via check, skipping...");
        return Ok(());
    }

    if existing_domains.contains(domain) {
        info!("│  ⚠️  Domain '{domain}' already exists, skipping...");
        return Ok(());
    }
    info!("│  🌍 Adding domain: {domain}");

    let account_id = &config.account_id;
    let project_name = &config.project_name;
    let add_domain_url = format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/pages/projects/{project_name}/domains");

    let payload = serde_json::json!({
        "name": domain
    });

    let response = client
        .post(&add_domain_url)
        .header(
            "Authorization",
            format!("Bearer {api}", api = config.api_token),
        )
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| GenWebBlogError::network(format!("Failed to send add domain request: {e}")))?;

    if response.status().is_success() {
        info!("│  ✅ Successfully added: {domain}");
        info!("│  🔄 Verifying {domain}");
    } else {
        let error_json: serde_json::Value = response
            .json()
            .await
            .unwrap_or_else(|_| serde_json::json!({ "errors": [{"message": "Unknown error"}]}));

        let error_message = error_json["errors"]
            .as_array()
            .and_then(|errors| errors.first())
            .and_then(|err| err["message"].as_str())
            .unwrap_or("Unknown error");

        warn!("│  ❌ Failed to add {domain}: {error_message}");
    }

    Ok(())
}

async fn is_domain_already_added(
    client: &Client,
    config: &CloudflareConfig,
    domain: &str,
) -> Result<bool> {
    let account_id = &config.account_id;
    let project_name = &config.project_name;
    let check_url = format!("https://api.cloudflare.com/client/v4/accounts/{account_id}/pages/projects/{project_name}/domains");

    let response = client
        .get(&check_url)
        .header(
            "Authorization",
            format!("Bearer {api}", api = config.api_token),
        )
        .send()
        .await
        .map_err(|e| GenWebBlogError::network(format!("Failed to request domains: {e}")))?;

    if response.status().is_success() {
        let json: Value = response
            .json()
            .await
            .unwrap_or_else(|_| serde_json::json!({}));
        if let Some(domains) = json["result"].as_array() {
            return Ok(domains.iter().any(|d| d["name"].as_str() == Some(domain)));
        }
    }

    Ok(false)
}
