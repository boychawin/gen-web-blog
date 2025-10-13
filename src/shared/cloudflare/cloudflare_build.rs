use crate::deploy::DeployConfig;
use crate::shared::error::GenWebBlogError;
use eyre::Result;
use log::{error, info, warn};
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};

#[allow(clippy::too_many_lines)]
pub async fn trigger_cloudflare_build_deploy(config: &DeployConfig<'_>) -> Result<()> {
    info!("│  🔍 Verifying Cloudflare API Token...");
    let token_verify_url = "https://api.cloudflare.com/client/v4/user/tokens/verify";
    let token_verify_response = config
        .client
        .get(token_verify_url)
        .header(
            "Authorization",
            format!("Bearer {}", config.cloudflare_api_token),
        )
        .header("User-Agent", "Rust-Deploy-Bot/1.0")
        .send()
        .await?;

    if !token_verify_response.status().is_success() {
        let err_text = token_verify_response.text().await.unwrap_or_default();
        error!("│  ❌ Invalid Cloudflare API Token: {err_text}");
        return Err(GenWebBlogError::cloudflare(format!(
            "Invalid Cloudflare API Token: {}",
            err_text
        ))
        .into());
    }
    info!("│  ✅ Cloudflare API Token is Valid!");

    info!("│  🔍 Checking Cloudflare Pages Project...");
    let check_url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}",
        config.cloudflare_account_id, config.project_name
    );

    let response = config
        .client
        .get(&check_url)
        .header(
            "Authorization",
            format!("Bearer {}", config.cloudflare_api_token),
        )
        .header("User-Agent", "Rust-Deploy-Bot/1.0")
        .send()
        .await?;

    if response.status().is_success() {
        info!(
            "│  ✅ Project '{}' Found! Proceeding with Deployment...",
            config.project_name
        );
    } else {
        warn!(
            "│  ⚠️ Cloudflare Pages project '{}' not found! Creating...",
            config.project_name
        );

        let create_url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects",
            config.cloudflare_account_id
        );

        let project_payload = json!({
            "name": config.project_name,
            "production_branch": config.branch,
            "source": {
                "type": "github",
                "config": {
                    "owner": config.user,
                    "repo_name": config.repo_name,
                    "production_branch": config.branch,
                    "env": {
                        "NODE_ENV": "production"
                    }
                }
            }
        });

        let create_response = config
            .client
            .post(&create_url)
            .header(
                "Authorization",
                format!("Bearer {}", config.cloudflare_api_token),
            )
            .header("User-Agent", "Rust-Deploy-Bot/1.0")
            .json(&project_payload)
            .send()
            .await?;

        if create_response.status().is_success() {
            info!(
                "│  ✅ Cloudflare Pages project '{}' created successfully!",
                config.project_name
            );
        } else {
            let err_text = create_response.text().await.unwrap_or_default();
            error!("│  ❌ Failed to create Cloudflare Pages project: {err_text}");
            return Err(GenWebBlogError::cloudflare(format!(
                "Failed to create Cloudflare Pages project: {}",
                err_text
            ))
            .into());
        }
    }

    let branch_exists = config
        .client
        .get(format!(
            "https://api.github.com/repos/{}/{}/branches/{}",
            config.user, config.repo_name, config.branch
        ))
        .header("Authorization", format!("Bearer {}", config.github_token))
        .header("User-Agent", "Rust-Deploy-Bot/1.0")
        .send()
        .await?
        .status()
        .is_success();

    if !branch_exists {
        error!(
            "│  ❌ The branch '{}' does not exist. Please provide a valid branch.",
            config.branch
        );
        return Err(GenWebBlogError::deploy(format!(
            "The branch '{}' does not exist",
            config.branch
        ))
        .into());
    }

    info!("│  🔄 Syncing Cloudflare Pages with GitHub...");

    sleep(Duration::from_secs(5)).await;

    let github_url = format!(
        "https://api.github.com/repos/{}/{}/dispatches",
        config.user, config.repo_name
    );

    let github_payload = json!({
        "event_type": "cloudflare-pages-deploy",
        "client_payload": { "branch": config.branch }
    });

    for _ in 0..3 {
        let github_response = config
            .client
            .post(&github_url)
            .header("Authorization", format!("Bearer {}", config.github_token))
            .header("Accept", "application/vnd.github.everest-preview+json")
            .header("User-Agent", "Rust-Deploy-Bot/1.0")
            .json(&github_payload)
            .send()
            .await?;

        if github_response.status().is_success() {
            info!("●  🎉 Deploy successful!");
            match get_deploy_url(
                config.client,
                config.cloudflare_account_id,
                config.project_name,
                config.cloudflare_api_token,
            )
            .await
            {
                Some(deploy_url) => {
                    info!("◆  🚀 Preview URL: {deploy_url}\n");
                }
                None => {
                    warn!("│  ❌ Failed to retrieve deployment link.\n");
                }
            }

            return Ok(());
        }
        let err_text = github_response.text().await.unwrap_or_default();
        warn!("│  ⚠️ Retrying GitHub repository dispatch due to error: {err_text}");
        sleep(Duration::from_secs(3)).await;
    }

    error!("│  ❌ Failed to trigger GitHub repository dispatch after retries.");

    Ok(())
}

pub async fn get_deploy_url(
    client: &Client,
    cloudflare_account_id: &str,
    project_name: &str,
    cloudflare_api_token: &str,
) -> Option<String> {
    let deploy_url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{cloudflare_account_id}/pages/projects/{project_name}/deployments"
    );

    for _ in 0..3 {
        let deploy_response = client
            .get(&deploy_url)
            .header("Authorization", format!("Bearer {cloudflare_api_token}"))
            .header("User-Agent", "Rust-Deploy-Bot/1.0")
            .send()
            .await;

        match deploy_response {
            Ok(response) => {
                if response.status().is_success() {
                    let deploy_response_text =
                        response.text().await.unwrap_or_else(|_| String::new());

                    if let Ok(json) =
                        serde_json::from_str::<serde_json::Value>(&deploy_response_text)
                    {
                        if let Some(url) = json["result"][0]["url"].as_str() {
                            return Some(url.to_string());
                        }
                        warn!("│  ❌ No URL found in the response.");
                    } else {
                        warn!("│  ❌ Failed to parse JSON response.");
                    }
                } else {
                    warn!("│  ❌ Received non-success status: {}", response.status());
                }
            }
            Err(_) => {
                warn!("│  ⚠️ Error occurred. Retrying...");
            }
        }

        sleep(Duration::from_secs(3)).await;
    }

    None
}
