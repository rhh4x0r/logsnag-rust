use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use lazy_regex::regex;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TagHashMap {
    tags: HashMap<String, String>,
}

impl TagHashMap {
    pub fn new() -> Self {
        Self { tags: HashMap::new() }
    }

    pub fn insert<T: AsRef<str>, U: AsRef<str>>(&mut self, key: T, value: U) {
        //Remove special characters, spaces, numbers, and lowercase everything for LogSnag API constraints
        let re = regex!(r"[^a-zA-Z\s-]");
        let key = re.replace_all(key.as_ref(), "");
        let value = re.replace_all(value.as_ref(), "");

        let re_space = regex!(r"\s");
        let key = re_space.replace_all(&key, "-");
        let value  = re_space.replace_all(&value, "-");

        let key_validated = key.to_lowercase();
        let value_validated = value.to_lowercase();

        self.tags.insert(key_validated, value_validated);
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

impl Log {
    pub fn new<T: AsRef<str>, U: AsRef<str>, V: AsRef<str>>(project: T, channel: U, event: V) -> Log {
        Log {
            project: project.as_ref().to_owned(),
            channel: channel.as_ref().to_owned(),
            event: event.as_ref().to_owned(),
            description: None,
            icon: None,
            notify: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum InsightValue{
    Str(String),
    Int(i32),
    Bool(bool)
}

impl From<&str> for InsightValue {
    fn from(value: &str) -> Self {
        InsightValue::Str(value.to_string())
    }
}

impl From<String> for InsightValue {
    fn from(value: String) -> Self {
        InsightValue::Str(value)
    }
}

impl From<i32> for InsightValue {
    fn from(value: i32) -> Self {
        InsightValue::Int(value)
    }
}

impl From<bool> for InsightValue {
    fn from(value: bool) -> Self {
        InsightValue::Bool(value)
    }
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
    pub icon: Option<String>,
}

impl Insight {
    pub fn new<T: AsRef<str>, U: AsRef<str>, V: Into<InsightValue>>(project: T, title: U, value: V) -> Insight {
        Insight {
            project: project.as_ref().to_owned(),
            title: title.as_ref().to_owned(),
            value: value.into(),
            icon: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_token: String,
    pub project: String,
}

impl Config {
    pub fn new<T: AsRef<str>, U: AsRef<str>>(api_token: T, project: U) -> Config {
        Config {
            api_token: api_token.as_ref().to_owned(),
            project: project.as_ref().to_owned()
        }
    }
}

