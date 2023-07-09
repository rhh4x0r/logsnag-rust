pub mod client;
pub mod models;

use models::{ Log, Insight, InsightValue, TagHashMap, Config};
use client::Client;

use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

use anyhow::Error;
use anyhow::anyhow;

const PUBLISH_API_URL: &str = "https://api.logsnag.com/v1/log";
const INSIGHT_API_URL: &str = "https://api.logsnag.com/v1/insight";

pub struct EventBuilder<'a> {
    logsnag: &'a Logsnag,
    log: Log
}

impl EventBuilder<'_> {
    pub fn with_description<T: AsRef<str>>(mut self, description: T) -> Self {
        self.log.description = Some(description.as_ref().to_owned());
        self
    }

    pub fn with_icon<T: AsRef<str>>(mut self, icon: T) -> Self {
        self.log.icon = Some(icon.as_ref().to_owned());
        self
    }

    pub fn with_notify(mut self, notify: bool) -> Self {
        self.log.notify = Some(notify);
        self
    }

    pub fn with_tag<T: AsRef<str>, U: AsRef<str>>(mut self, tag_key: T, tag_value: U) -> Self {
        let tag_map = self.log.tags.get_or_insert_with(|| TagHashMap::new());
        tag_map.insert(tag_key.as_ref().to_owned(), tag_value.as_ref().to_owned());
        self
    }
    pub async fn publish(&self) -> Result<Response, Error>{
        let request_data = serde_json::to_value(self.log.to_owned())?;

        let response = self
            .logsnag
            .client
            .post(PUBLISH_API_URL)
            .json(&request_data)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.logsnag.config.api_token.to_owned())
            .send()
            .await?;

        if response.status() == 200 {
            Ok(response)
        } else {
            Err(anyhow!("Error in response: {:?}", response.text().await))
        }
    }
}

pub struct InsightBuilder<'a> {
    logsnag: &'a Logsnag,
    insight: Insight
}

impl InsightBuilder<'_> {
    pub fn with_icon<T: AsRef<str>>(mut self, icon: T) -> Self{
        self.insight.icon = Some(icon.as_ref().to_owned());
        self
    }
    pub async fn publish(&self) -> Result<Response, Error>{
        let request_data = serde_json::to_value(self.insight.to_owned())?;

        let response = self
            .logsnag
            .client
            .post(INSIGHT_API_URL)
            .json(&request_data)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.logsnag.config.api_token.to_owned())
            .send()
            .await?;

        if response.status() == 200 {
            Ok(response)
        } else {
            Err(anyhow!("Error in response: {:?}", response.text().await))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Logsnag {
    pub config: Config,
    pub client: Client,
}

impl Logsnag {
    pub fn new<T: AsRef<str>, U: AsRef<str>>(api_token: T, project: U) -> Logsnag {
        Logsnag { 
            config: Config::new(api_token.as_ref(), project.as_ref()),
            client: Client::new(),
        }
    }

    pub fn event<T: AsRef<str>, U: AsRef<str>>(&self, channel: T, event: U) -> EventBuilder {
        let event_log = Log::new(&self.config.project.as_str(), channel.as_ref(), event.as_ref());
        EventBuilder {
            logsnag: self,
            log: event_log
        }
    }

    pub fn insight<T: AsRef<str>, U: Into<InsightValue>>(&self, title: T, value: U) -> InsightBuilder {
        let insight_log = Insight::new(&self.config.project.as_str(), title.as_ref(), value.into());
        InsightBuilder {
            logsnag: self,
            insight: insight_log
        }
    }
}