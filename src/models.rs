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

    pub fn insert(&mut self, key: &str, value: &str) {
        //Remove special characters, spaces, numbers, and lowercase everything for LogSnag API constraints
        let re = regex!(r"[^a-zA-Z\s-]");
        let key = re.replace_all(key, "");
        let value = re.replace_all(value, "");

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
pub enum InsightValue<'a> {
    Str(&'a str),
    Int(i32),
    Bool(bool)
}

impl<'a> From<&'a str> for InsightValue<'a> {
    fn from(value: &'a str) -> Self {
        InsightValue::Str(value)
    }
}

impl<'a> From<i32> for InsightValue<'a> {
    fn from(value: i32) -> Self {
        InsightValue::Int(value)
    }
}

impl<'a> From<bool> for InsightValue<'a> {
    fn from(value: bool) -> Self {
        InsightValue::Bool(value)
    }
}

impl Serialize for InsightValue<'_> {
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
    pub value: InsightValue<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<&'a str>
}

impl<'a> Insight<'a> {
    pub fn new(project: &'a str, title: &'a str, value: InsightValue<'a>) -> Insight<'a> {
        Insight {
            project,
            title,
            value,
            icon: None,
        }
    }
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


