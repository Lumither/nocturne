use serde_json::Value;
use std::str::FromStr;
use yaml_rust2::Yaml;

pub fn yaml_to_serde(yaml: &Yaml) -> Value {
    match yaml {
        Yaml::Null => Value::Null,
        Yaml::Boolean(b) => Value::Bool(*b),
        Yaml::Integer(i) => Value::Number(serde_json::Number::from(*i)),
        Yaml::Real(s) => Value::Number(serde_json::Number::from_str(s).unwrap()),
        Yaml::String(s) => Value::String(s.clone()),
        Yaml::Array(arr) => Value::Array(arr.iter().map(yaml_to_serde).collect()),
        Yaml::Hash(hash) => {
            let obj: serde_json::map::Map<String, Value> = hash
                .iter()
                .map(|(k, v)| {
                    let key = match k {
                        Yaml::String(s) => s.clone(),
                        _ => "INVALID_KEY".to_string(),
                    };
                    (key, yaml_to_serde(v))
                })
                .collect();
            Value::Object(obj)
        }
        Yaml::Alias(_) => Value::Null,
        Yaml::BadValue => Value::Null,
    }
}
