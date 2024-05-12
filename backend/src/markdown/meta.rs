use std::collections::HashMap;

use regex::Regex;

/// return the meta info contained in the .md file front matter
pub fn parse_meta(md_content: &str) -> HashMap<String, String> {
    let mut fields: HashMap<String, String> = HashMap::new();
    let match_regex = Regex::new(r"(?s)---\n(.*\n)---\n").unwrap();
    if let Some(front_matter) = match_regex.captures(md_content) {
        let fm_content = front_matter.get(1).unwrap().as_str();
        for line in fm_content.lines() {
            if let Some((k, v)) = extract_kv(line) {
                fields.insert(k, v);
            }
        }
    }
    fields
}

fn extract_kv(line: &str) -> Option<(String, String)> {
    let split: Vec<&str> = line.splitn(2, ':').map(|s| s.trim()).collect();
    if split.len() == 2 {
        let key = split[0].to_string();
        let value = split[1].to_string();
        Some((key, value))
    } else {
        None
    }
}

#[cfg(test)]
mod meta_test {
    use std::error::Error;

    use crate::markdown::meta::parse_meta;

    #[test]
    fn test_parse_meta_front_matter() -> Result<(), Box<dyn Error>> {
        let parsed_meta = parse_meta(
            r##"
---
title: Hello World!
tags: [ "test", "hello_world" ]
---
            "##,
        );
        dbg!(&parsed_meta);
        assert_eq!(parsed_meta.get("title"), Some(&"Hello World!".to_string()));
        assert_eq!(
            parsed_meta.get("tags"),
            Some(&r#"[ "test", "hello_world" ]"#.to_string())
        );
        Ok(())
    }
}
