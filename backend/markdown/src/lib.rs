pub mod error;
mod utils;

use std::{fs, path::Path, str::FromStr};

use crate::{
    error::Error,
    utils::front_matter::{parse_front_matter, split_md_front_matter},
};

use base64::{Engine, engine::general_purpose};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Default, Debug, Serialize, Clone)]
pub struct MdFile {
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
        Self::from_str(&raw_file)
    }
}

impl FromStr for MdFile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes().to_vec();
        let (front_matter_str, content) = split_md_front_matter(s);
        let front_matter = match parse_front_matter(&front_matter_str) {
            Ok(front_matter) => front_matter,
            Err(e) => return Err(Error::InvalidFrontMatter { msg: e.to_string() }),
        };
        let hash = calculate_hash_base64(&bytes);
        Ok(MdFile {
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
