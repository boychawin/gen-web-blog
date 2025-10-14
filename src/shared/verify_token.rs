use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use block_padding::Pkcs7;
use cbc::{Decryptor, Encryptor};
use directories::ProjectDirs;
use hex::{decode, encode};
use log::{info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

type Aes128CbcEnc = Encryptor<Aes128>;
type Aes128CbcDec = Decryptor<Aes128>;

#[derive(Serialize, Deserialize)]
struct TokenData {
    token: String,
    expiration: u64,
}

const SECRET_KEY: &[u8; 16] = b"verysecretkey123";
const IV: &[u8; 16] = b"uniqueinitvector";

fn get_token_cache_path() -> io::Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "genwebblog", "ClientTool") {
        let dir = proj_dirs.config_dir();
        fs::create_dir_all(dir)?;

        Ok(dir.join("token_cache.json"))
    } else {
        Err(io::Error::other("Failed to get cache path"))
    }
}

fn encrypt_data(data: &[u8]) -> io::Result<String> {
    let cipher = Aes128CbcEnc::new_from_slices(SECRET_KEY, IV)
        .map_err(|e| io::Error::other(format!("Invalid key/IV: {e}")))?;

    let mut buffer = data.to_vec();
    let len = buffer.len();
    buffer.resize(len + 16, 0);

    let encrypted_data = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, len)
        .map_err(|e| io::Error::other(format!("Encryption failed: {e}")))?;
    Ok(encode(encrypted_data))
}

fn decrypt_data(encrypted_data: &str) -> io::Result<Vec<u8>> {
    let cipher = Aes128CbcDec::new_from_slices(SECRET_KEY, IV)
        .map_err(|e| io::Error::other(format!("Invalid key/IV: {e}")))?;

    let decoded_data = decode(encrypted_data).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Hex decode failed: {e}"),
        )
    })?;

    let mut buf = decoded_data.clone();
    let decrypted = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Decryption failed: {e}"),
        )
    })?;
    Ok(decrypted.to_vec())
}

fn parse_token_data(decrypted_contents: &[u8]) -> io::Result<TokenData> {
    serde_json::from_slice(decrypted_contents).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid token JSON: {e}"),
        )
    })
}

pub async fn verify_token(token: &str) -> io::Result<()> {
    let token_file = match get_token_cache_path() {
        Ok(p) => p,
        Err(e) => {
            warn!("Failed to get token cache path, falling back to cwd: {e}");
            PathBuf::from("token_cache.json")
        }
    };

    if let Ok(contents) = crate::shared::fs::read_file_to_string(&token_file) {
        match decrypt_data(&contents) {
            Ok(decrypted_contents) => match parse_token_data(&decrypted_contents) {
                Ok(token_data) => {
                    let current_time = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map_err(io::Error::other)?
                        .as_secs();
                    if token_data.token == token && token_data.expiration > current_time {
                        info!("|  ✅ Token valid (cached).");
                        return Ok(());
                    }
                }
                Err(e) => warn!("Failed to parse cached token: {e}"),
            },
            Err(e) => warn!("Failed to decrypt cached token: {e}"),
        }
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| io::Error::other(format!("Failed to build HTTP client: {e}")))?;

    let response = client
        .post("https://api.genwebblog.com/verify_token")
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await
        .map_err(|e| io::Error::other(format!("HTTP request failed: {e}")))?;

    if response.status().is_success() {
        info!("|  ✅ Token verified successfully.");

        let expiration = SystemTime::now() + Duration::from_secs(30 * 24 * 60 * 60);
        let expiration_timestamp = expiration
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_secs();

        let token_data = TokenData {
            token: token.to_string(),
            expiration: expiration_timestamp,
        };

        let token_json = serde_json::to_vec(&token_data)
            .map_err(|e| io::Error::other(format!("Serialize token failed: {e}")))?;
        let encrypted_data = encrypt_data(&token_json)?;

        fs::write(&token_file, encrypted_data)?;

        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Token verification failed: HTTP {status}",
                status = response.status()
            ),
        ))
    }
}
