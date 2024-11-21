mod utils;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::utils::front_matter::{parse_front_matter, split_md_front_matter};

#[derive(Default, Debug, Serialize)]
pub struct MdFile {
    pub file_id: Uuid,
    pub file_name: String,
    pub meta: Value,
    pub content: String,
    pub hash: String,
}

impl MdFile {
    pub async fn from_file(file_path: &str) -> Result<MdFile, Box<dyn Error>> {
        let raw_file = fs::read_to_string(file_path)?;
        let raw_file_bytes = raw_file.as_bytes().to_vec();
        let (front_matter_str, content) = split_md_front_matter(raw_file);
        let front_matter = parse_front_matter(&front_matter_str)?;
        let hash = calculate_hash_base64(&raw_file_bytes);
        let file_id = Uuid::from_str(front_matter["id"].to_string().as_str())?;
        let file_name = Path::new(file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok(MdFile {
            file_id,
            file_name,
            meta: front_matter,
            content,
            hash,
        })
    }
}

fn calculate_hash_base64(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hasher.finalize().to_vec();
    general_purpose::STANDARD.encode(hash)
}
