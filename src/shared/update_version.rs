use eyre::Result;
use reqwest::blocking::get;
use serde::Deserialize;
use std::fs::File;
use std::io::copy;
use std::process::Command;
use tokio::task::block_in_place;
use log::info;

#[derive(Deserialize)]
struct ReleaseInfo {
    version: String,
}

fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let parts: Vec<&str> = v.trim().trim_start_matches('v').split('.').collect();
    let major = parts.get(0).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

pub async fn check_for_update(current_version: &str) -> Result<()> {
    let url = "https://genwebblog.com/releases.json";

    let response: ReleaseInfo = block_in_place(|| {
        let resp = get(url)?;
        if !resp.status().is_success() {
            return Err(eyre::eyre!("Failed to fetch releases.json: HTTP {}", resp.status()));
        }
        resp.json().map_err(|e| eyre::eyre!("Failed to parse releases.json: {}", e))
    })?;

    let latest_version = response.version;

    let should_update = if let (Some(curr), Some(latest)) = (parse_version(current_version), parse_version(&latest_version)) {
        latest > curr
    } else {
        latest_version.as_str() > current_version
    };

    if should_update {
        info!("│  🔔 New version available: {latest_version}");
        let download_url = format!("https://genwebblog.com/releases/{latest_version}.zip");
        info!("│  📥 Downloading from: {download_url}");

        block_in_place(|| {
            let mut response = get(&download_url)?;
            if !response.status().is_success() {
                return Err(eyre::eyre!("Failed to download update: HTTP {}", response.status()));
            }
            let mut out_file = File::create("update.zip")?;
            copy(&mut response, &mut out_file)?;
            Ok::<(), eyre::Report>(())
        })?;

        info!("│  ✅ Download completed. Applying update...");
        block_in_place(|| {
            let status = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args([
                        "Expand-Archive",
                        "-Path",
                        "update.zip",
                        "-DestinationPath",
                        ".",
                    ])
                    .status()
            } else {
                Command::new("unzip").args(["-o", "update.zip", "-d", "."]).status()
            }?;

            if !status.success() {
                return Err(eyre::eyre!("Extraction command failed with status: {}", status));
            }

            Ok::<(), eyre::Report>(())
        })?;

        info!("│  🎉 Update applied successfully!");
    } else {
        info!("│  ✅ Already on the latest version");
    }

    Ok(())
}
