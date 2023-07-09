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
pub struct Log<'a> {
    pub project: &'a str,
    pub channel: &'a str,
    pub event: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagHashMap>,
}

impl<'a> Log<'a> {
    pub fn new(project: &'a str, channel: &'a str, event: &'a str) -> Log<'a> {
        Log {
            project,
            channel,
            event,
            description: None,
            icon: None,
            notify: None,
            tags: None,
        }
    }
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
pub struct Insight<'a> {
    pub project: &'a str,
    pub title: &'a str,
    pub value: InsightValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub api_token: &'a str,
    pub project: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(api_token: &'a str, project: &'a str) -> Config<'a> {
        Config {
            api_token: api_token,
            project: project
        }
    }
}


