use std::{env, fs};
use std::error::Error;
use std::process::Command;
use std::str;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use tracing::{error, warn};
use uuid::Uuid;

use crate::constants::GLOBAL_TIME_FORMAT;
use crate::markdown::meta::parse_meta;

pub fn from_path(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut res = Map::new();

    // field `content`
    let md_content = fs::read_to_string(path)?;
    res.insert("content".to_string(), Value::from(md_content.to_owned()));

    // field `meta`
    let meta = parse_meta(&md_content);
    let meta_map: Map<String, Value> = meta
        .clone()
        .into_iter()
        .map(|(k, v)| (k, Value::from(v)))
        .collect();
    res.insert("meta".to_string(), Value::from(meta_map));

    // field `post_id`
    let post_id = match meta.get("id") {
        None => Uuid::new_v4(),
        Some(id) => match Uuid::from_str(id.as_str()) {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to parse UUID <{}>: {}", id.as_str(), e.to_string());
                return Err(Box::new(e));
            }
        },
    };
    res.insert("post_id".to_string(), Value::from(post_id.to_string()));

    // field `title`
    let title = match meta.get("title") {
        None => "Title Not Found".to_string(),
        Some(title) => title.to_owned(),
    };
    res.insert("title".to_string(), Value::from(title));

    // field `summary`
    let summary = match meta.get("summary") {
        None => "".to_string(),
        Some(summary) => summary.to_owned(),
    };
    res.insert("summary".to_string(), Value::from(summary));

    // field `last_update`
    let git_output = match Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("-1")
        .arg("--format=%cI")
        .arg("--")
        .current_dir(env::var("GIT_WORK_DIR")?)
        .arg(&path)
        .output()
    {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to execute `git` command: {}", e.to_string());
            return Err(Box::new(e));
        }
    };
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

    // field `first_update`
    let first_update: DateTime<Utc> = match meta.get("date") {
        None => {
            warn!(
                "Failed to parse first update time for post {}, using current time instead",
                post_id.to_string()
            );
            Utc::now().to_utc()
        }
        Some(date) => DateTime::from(
            match DateTime::parse_from_str(format!("{}T000000", date).as_str(), "%Y-%m-%d%zT%H%M%S")
            {
                Ok(time) => time,
                Err(e) => {
                    warn!(
                        "Failed to parse first update time for post {}, using current time instead: {}",
                        post_id.to_string(),
                        e.to_string()
                    );
                    DateTime::from(Utc::now())
                }
            },
        ),
    };
    res.insert(
        "first_update".to_string(),
        Value::from(first_update.format(GLOBAL_TIME_FORMAT).to_string()),
    );

    Ok(res)
}
