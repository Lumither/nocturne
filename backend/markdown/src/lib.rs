mod error;
mod utils;

use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::error::Error;
use crate::utils::front_matter::{parse_front_matter, split_md_front_matter};

use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Default, Debug, Serialize)]
pub struct MdFile {
    pub file_id: Uuid,
    pub filename: String,
    pub meta: Value,
    pub content: String,
    pub hash: String,
}

impl MdFile {
    pub fn from_file(file_path: &Path) -> Result<MdFile, Error> {
        let raw_file = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(e) => return Err(Error::FSError { msg: e.to_string() }),
        };
        let raw_file_bytes = raw_file.as_bytes().to_vec();
        let (front_matter_str, content) = split_md_front_matter(raw_file);
        let front_matter = match parse_front_matter(&front_matter_str) {
            Ok(front_matter) => front_matter,
            Err(e) => return Err(Error::InvalidFrontMatter { msg: e.to_string() }),
        };
        let hash = calculate_hash_base64(&raw_file_bytes);
        let file_id = match front_matter["id"].as_str() {
            Some(id) => match Uuid::from_str(id) {
                Ok(id) => id,
                Err(e) => return Err(Error::InvalidID { msg: e.to_string() }),
            },
            None => return Err(Error::MissingID),
        };
        let file_name = Path::new(file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok(MdFile {
            file_id,
            filename: file_name,
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
