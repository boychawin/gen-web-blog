use std::process::Command;
use log::{info, error};

fn install_choco_if_missing() -> Result<(), String> {
    let choco_check = Command::new("where")
        .arg("choco")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !choco_check {
        info!("🛠 Installing Chocolatey...");
        let choco_install_cmd = "Set-ExecutionPolicy Bypass -Scope Process -Force; \
            [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; \
            iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))";

        let status = Command::new("powershell")
            .arg("-Command")
            .arg(choco_install_cmd)
            .status()
            .map_err(|e| format!("❌ Failed to install Chocolatey: {e}"))?;

        if !status.success() {
            error!("❌ Chocolatey installation failed.");
            return Err("Chocolatey installation failed".to_string());
        }
    }
    Ok(())
}

pub fn install_required_tool(command: &str) -> Result<(), String> {

    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        error!("❌ Unsupported operating system");
        return Err("Unsupported operating system".to_string());
    };

    info!("🔄 Installing {command} on {os}...");

    match os {
        "linux" => {
            let install_command = match command {
                "inkscape" => "sudo apt install -y inkscape",
                "magick" => "sudo apt install -y imagemagick",
                "ffmpeg" => "sudo apt install -y ffmpeg",
                _ => {
                    error!("❌ Unsupported tool for Linux");
                    return Err("Unsupported tool for Linux".to_string());
                }
            };

            Command::new("sh")
                .arg("-c")
                .arg(install_command)
                .status()
                .map_err(|e| format!("❌ Error installing {command}: {e}"))?;
        }
        "macos" => {
            let brew_check = Command::new("which")
                .arg("brew")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);

            if !brew_check {
                info!("🛠 Installing Homebrew...");
                let brew_install_cmd = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(brew_install_cmd)
                    .status()
                    .map_err(|e| format!("❌ Failed to install Homebrew: {e}"))?;

                if !status.success() {
                    error!("❌ Homebrew installation failed.");
                    return Err("Homebrew installation failed".to_string());
                }
            }

            let install_command = match command {
                "inkscape" => "brew install inkscape",
                "magick" => "brew install imagemagick",
                "ffmpeg" => "brew install ffmpeg",
                _ => {
                    error!("❌ Unsupported tool for macOS");
                    return Err("Unsupported tool for macOS".to_string());
                }
            };

            Command::new("sh")
                .arg("-c")
                .arg(install_command)
                .status()
                .map_err(|e| format!("❌ Error installing {command}: {e}"))?;
        }
        "windows" => {
            install_choco_if_missing()?;

            let install_command = match command {
                "inkscape" => "choco install -y inkscape",
                "magick" => "choco install -y imagemagick",
                "ffmpeg" => "choco install -y ffmpeg",
                _ => {
                    error!("❌ Unsupported tool for Windows");
                    return Err("Unsupported tool for Windows".to_string());
                }
            };

            Command::new("cmd")
                .arg("/C")
                .arg(install_command)
                .status()
                .map_err(|e| format!("❌ Error installing {command}: {e}"))?;
        }
        _ => {
            error!("❌ Unsupported OS");
            return Err("Unsupported OS".to_string());
        }
    }
    info!("✅ {command} installed successfully!");
    Ok(())
}
