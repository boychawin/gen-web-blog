#[warn(deprecated)]
use crate::shared::cloudflare::cloudflare_build::trigger_cloudflare_build_deploy;
use base64::{engine::general_purpose, Engine as _};
use eyre::{eyre, Result};
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository};
use reqwest::Client;
use crate::shared::github::build_github_client;
use log::{info, warn, error};
use serde_json::json;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct DeployConfig<'a> {
    pub user: &'a str,
    pub repo_name: &'a str,
    pub branch: &'a str,
    pub build_dir: &'a str,
    pub github_token: &'a str,
    pub client: &'a Client,
    pub cloudflare_api_token: &'a str,
    pub cloudflare_account_id: &'a str,
    pub project_name: &'a str,
}

pub async fn check_repo_exists(
    client: &Client,
    github_token: &str,
    github_user: &str,
    github_repo: &str,
    github_private: &bool,
) -> Result<bool> {
    let url = format!("https://api.github.com/repos/{github_user}/{github_repo}");
    let gh = build_github_client(client, github_token);
    let response = gh.get(&url).await?;

    if !response.status().is_success() {
        info!("🔧 Repository not found, creating a new repository...");
        create_repo_and_push(
            client,
            github_token,
            github_repo,
            github_user,
            github_private,
        )
        .await?;
    }

    Ok(response.status().is_success())
}

pub async fn create_repo_and_push(
    client: &Client,
    token: &str,
    github_repo: &str,
    github_user: &str,
    github_private: &bool,
) -> Result<()> {
    let repo_api_url = format!("https://api.github.com/repos/{github_user}/{github_repo}");
    let create_repo_url = "https://api.github.com/user/repos";

    let gh = build_github_client(client, token);
    let check_response = gh.get(&repo_api_url).await?;

    if check_response.status().is_success() {
        info!("│  ✅ Repository '{github_repo}' already exists on GitHub!");
    } else {
        let create_response = gh
            .post_json(create_repo_url, &json!({ "name": github_repo, "private": github_private }))
            .await?;

        if create_response.status().is_success() {
            info!("│  ✅ Repository '{github_repo}' created on GitHub!");
        } else {
            let error_msg: String = create_response.text().await?;
            error!("│  ❌ Failed to create repository: {error_msg}");
            return Err(eyre!(error_msg));
        }
    }

    let branch_url = format!("{repo_api_url}/git/ref/heads/main");
    let branch_response = gh.get(&branch_url).await?;

    if branch_response.status().is_success() {
        let branch_data: serde_json::Value = branch_response.json().await?;
        branch_data
            .get("object")
            .and_then(|obj| obj.get("sha"))
            .and_then(|sha| sha.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        info!("│  ⚠️ Branch 'main' not found. Creating new branch...");
        String::new()
    };

    let file_content = general_purpose::STANDARD.encode("# Initial Commit\n");
    let create_file_url = format!("{repo_api_url}/contents/README.md");
    let create_file_response = gh
        .put_json(&create_file_url, &json!({
            "message": "Initial commit",
            "content": file_content,
            "branch": "main"
        }))
        .await?;

    if create_file_response.status().is_success() {
        info!("│  ✅ Pushed initial commit to GitHub!");
    } else {
        let error_msg: String = create_file_response.text().await?;
        error!("│  ❌ Failed to push initial commit: {error_msg}");
        return Err(eyre!(error_msg));
    }

    info!("│  🎉 Repository setup completed successfully!");
    Ok(())
}

fn scan_dir(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_dir(&path);
            } else {
                info!("│  📄 Found file: {}", path.display());
            }
        }
    }
}

pub async fn push_to_github(config: &DeployConfig<'_>) -> Result<()> {
    let repo_path = Path::new(config.build_dir);

    let repo = if let Ok(repo) = Repository::discover(repo_path) {
        repo
    } else {
        info!(
            "│  ⚠️ ไม่พบ Git Repository ใน '{}', กำลังสร้างใหม่...",
            config.build_dir
        );
        Repository::init(repo_path)?
    };

    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
    let mut index = repo.index()?;

    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    if index.is_empty() {
        info!("│  ⚠️ No changes to commit. Skipping commit.");
        return Ok(());
    }

    info!("│  🔍 Scanning files after staging...");
    scan_dir(Path::new(config.build_dir));

    let tree_id: git2::Oid = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = repo.signature()?;

    let commit_id = match parent_commit {
        Some(parent) => repo.commit(Some("HEAD"), &sig, &sig, "new commit", &tree, &[&parent])?,
        None => repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?,
    };

    info!("│  ✅ Commit Created: {commit_id}");

    let commit = repo.find_commit(commit_id)?;
    if let Ok(mut reference) = repo.find_reference(&format!("refs/heads/{}", config.branch)) {
        reference.set_target(commit_id, "Update HEAD")?;
    } else {
        info!(
            "│  ⚠️ Branch '{}' not found, creating new branch...",
            config.branch
        );
        repo.branch(config.branch, &commit, false)?;
    }

    push_to_github_remote(&repo, config).await?;

    Ok(())
}

async fn push_to_github_remote(repo: &Repository, config: &DeployConfig<'_>) -> Result<()> {
    let remote_url = format!(
        "https://github.com/{}/{}.git",
        config.user, config.repo_name
    );

    let mut remote = if let Ok(remote) = repo.find_remote("origin") {
        let url_matches = remote.url().is_some_and(|url| url == remote_url);
        if !url_matches {
            info!("│  ⚠️ Remote 'origin' exists but URL does not match. Updating URL...");
            repo.remote_set_url("origin", &remote_url)?;
        }
        remote
    } else {
        info!("│  ⚠️ ไม่พบ Remote 'origin', กำลังเพิ่มใหม่...");
        repo.remote("origin", &remote_url)?
    };

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("x-access-token", config.github_token)
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    info!("│  🚀 Pushing to GitHub...");
    match remote.push(
        &[format!(
            "+refs/heads/{}:refs/heads/{}",
            config.branch, config.branch
        )],
        Some(&mut push_options),
    ) {
        Ok(()) => {
            info!("│  🎉 Successfully Pushed to GitHub!\n");

            trigger_cloudflare_build_deploy(config).await?;

            Ok(())
        }
        Err(e) => {
            error!("│  ❌ Failed to push to GitHub: {e}");
            return Err(eyre!(format!("Failed to push to GitHub: {e}")));
        }
    }
}