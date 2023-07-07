use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TagHashMap {
    tags: HashMap<String, String>,
}

impl TagHashMap {
    pub fn new() -> Self {
        Self { tags: HashMap::new() }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        let key = key.to_lowercase().replace("_", "-").replace(" ", "-");
        let value = value.to_lowercase().replace("_", "-").replace(" ", "-");

        self.tags.insert(key, value.to_owned());
    }
}

impl Serialize for TagHashMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.tags.serialize(serializer)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Log {
    pub project: String,
    pub channel: String,
    pub event: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagHashMap>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum InsightValue {
    Str(String),
    Int(i32),
    Bool(bool)
}

impl Serialize for InsightValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            InsightValue::Str(ref s) => serializer.serialize_str(s),
            InsightValue::Int(i) => serializer.serialize_i32(i),
            InsightValue::Bool(b)=> serializer.serialize_bool(b),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Insight {
    pub project: String,
    pub title: String,
    pub value: InsightValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_token: String,
    pub project: String
}

impl Config {
    pub fn new(api_token: String, project: String) -> Config {
        Config {
            api_token: api_token,
            project: project
        }
    }
}