use serde::{Deserialize, Serialize};

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
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum InsightValue {
    Str(String),
    Int(i32),
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
    pub fn new(api_token: &str, project: &str) -> Config {
        Config {
            api_token: api_token.to_owned(),
            project: project.to_owned()
        }
    }
}