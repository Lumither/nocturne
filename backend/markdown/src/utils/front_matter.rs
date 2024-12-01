use crate::utils::yaml::yaml_to_serde;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use yaml_rust2::YamlLoader;

pub fn split_md_front_matter(raw_file: String) -> (String, String) {
    let re = Regex::new(r"(?s)^(---\n.*?\n---\n)(.*)").unwrap();
    if let Some(captures) = re.captures(raw_file.as_str()) {
        let front_matter = captures.get(1).map_or("", |m| m.as_str()).to_string();
        let main_content = captures.get(2).map_or("", |m| m.as_str()).to_string();
        (front_matter, main_content)
    } else {
        (String::new(), raw_file)
    }
}

pub fn parse_front_matter(front_matter: &str) -> Result<Value, Box<dyn Error>> {
    let parsed_f_matter = YamlLoader::load_from_str(front_matter)?;
    if parsed_f_matter.is_empty() {
        Ok(Value::String(String::new()))
    } else {
        Ok(yaml_to_serde(&parsed_f_matter[0]))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::front_matter::{parse_front_matter, split_md_front_matter};

    #[test]
    fn front_matter_to_string() {
        let (front_matter, main_content) = split_md_front_matter(
            r##"---
title: test_title
date: 0000-00-00+0000
category: test
id: bf03afd8-61b4-4e78-a34a-8b3ee39adf16
sub_title: sub_title
summary: summary
tags: [ "test1", "test2" ]
---

test body

"##
            .to_string(),
        );
        assert_eq!(
            front_matter,
            r##"---
title: test_title
date: 0000-00-00+0000
category: test
id: bf03afd8-61b4-4e78-a34a-8b3ee39adf16
sub_title: sub_title
summary: summary
tags: [ "test1", "test2" ]
---
"##
        );
        assert_eq!(main_content, "\ntest body\n\n");
    }

    #[test]
    fn test_front_matter_parser() {
        dbg!(parse_front_matter(
            r##"---
title: test_title
date: 0000-00-00+0000
category: test
id: bf03afd8-61b4-4e78-a34a-8b3ee39adf16
sub_title: sub_title
summary: summary
tags: [ "test1", "test2" ]
---
"##,
        ))
        .expect("Failed to parse front matter");
    }
}
