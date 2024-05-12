use std::{env, fs};
use std::error::Error;
use std::process::Command;
use std::str;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::constants::GLOBAL_TIME_FORMAT;
use crate::markdown::meta::parse_meta;

pub fn from_path(path: String) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut res = Map::new();

    let md_content = fs::read_to_string(&path)?;
    res.insert("content".to_string(), Value::from(md_content.to_owned()));

    let meta = parse_meta(&md_content);
    let meta_map: Map<String, Value> = meta
        .clone()
        .into_iter()
        .map(|(k, v)| (k, Value::from(v)))
        .collect();
    res.insert("meta".to_string(), Value::from(meta_map));

    let post_id = match meta.get("id") {
        None => Uuid::new_v4(),
        Some(id) => Uuid::from_str(id.as_str()).expect("Invalid UUID String"),
    };
    res.insert("post_id".to_string(), Value::from(post_id.to_string()));

    let title = match meta.get("title") {
        None => "Title Not Found".to_string(),
        Some(title) => title.to_owned(),
    };
    res.insert("title".to_string(), Value::from(title));

    let summary = match meta.get("summary") {
        None => "".to_string(),
        Some(summary) => summary.to_owned(),
    };
    res.insert("summary".to_string(), Value::from(summary));

    let git_output = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("-1")
        .arg("--format=%cI")
        .arg("--")
        .current_dir(env::var("GIT_WORK_DIR")?)
        .arg(&path)
        .output()
        .expect("Failed to execute git command");
    let last_update: DateTime<Utc> = if git_output.status.success() {
        chrono::DateTime::parse_from_str(
            str::from_utf8(&git_output.stdout).unwrap().trim_end(),
            GLOBAL_TIME_FORMAT,
        )
        .unwrap_or_else(|_| DateTime::from(Utc::now()))
        .to_utc()
    } else {
        Utc::now().to_utc()
    };
    res.insert(
        "last_update".to_string(),
        Value::from(last_update.format(GLOBAL_TIME_FORMAT).to_string()),
    );

    let first_update: DateTime<Utc> = match meta.get("date") {
        None => Utc::now().to_utc(),
        Some(date) => DateTime::from(
            DateTime::parse_from_str(format!("{}T000000", date).as_str(), "%Y-%m-%d%zT%H%M%S")
                .expect("Failed to parse first_update time"),
        ),
    };
    res.insert(
        "first_update".to_string(),
        Value::from(first_update.format(GLOBAL_TIME_FORMAT).to_string()),
    );

    Ok(res)
}
