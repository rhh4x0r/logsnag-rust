pub mod models;
pub mod client;

use models::{ Log, Insight, InsightValue, TagHashMap, Config};
use client::Client;

use lazy_regex::regex_is_match;

use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

use anyhow::Error;
use anyhow::anyhow;

//TODO change to v1/ and then add /log or /insights depending on what they want
const PUBLISH_API_URL: &str = "https://api.logsnag.com/v1/log";
const INSIGHT_API_URL: &str = "https://api.logsnag.com/v1/insight";

pub struct EventBuilder<'a> {
    logsnag: &'a Logsnag<'a>,
    log: Log<'a>
}

impl<'a> EventBuilder<'a> {

    pub fn with_description(&'a mut self, description: &'a str) -> &'a mut Self{
        self.log.description = Some(description);
        self
    }

    pub fn with_icon(&'a mut self, icon: &'a str) -> &'a mut Self{
        self.log.icon = Some(icon);
        self
    }

    pub fn with_notify(&'a mut self, notify: bool) -> &'a mut Self{
        self.log.notify = Some(notify);
        self
    }

    pub fn with_tag(&'a mut self, tag_key: &str, tag_value: &str) -> &'a mut Self {
        let tag_map = self.log.tags.get_or_insert_with(|| TagHashMap::new());
        tag_map.insert(tag_key, tag_value);

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
            .bearer_auth(self.logsnag.config.api_token)
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
    logsnag: &'a Logsnag<'a>,
    insight: Insight<'a>
}

impl<'a> InsightBuilder<'a> {

    pub fn with_icon(&'a mut self, icon: &'a str) -> &'a mut Self{
        self.insight.icon = Some(icon);
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
            .bearer_auth(self.logsnag.config.api_token)
            .send()
            .await?;

        if response.status() == 200 {
            Ok(response)
        } else {
            Err(anyhow!("Error in response: {:?}", response.text().await))
        }
    }
}
/// `Logsnag` is a struct used to interact with the Logsnag API.
/// It contains the configuration and client needed to make requests.
#[derive(Clone, Debug)]
pub struct Logsnag<'a> {
    pub config: Config<'a>,
    pub client: Client,
}

impl<'a> Logsnag<'a> {

    /// This method creates a new instance of a Logsnag client.
    ///
    /// # Arguments
    ///
    /// * `api_token` - A string slice that holds the API token.
    /// * `project` - A string slice that holds the name of the project.
    ///
    /// # Examples
    ///
    /// ```
    /// use logsnag::Logsnag;
    ///
    /// let client = Logsnag::new("my-api-token", "my-project");
    /// ```
    pub fn new(api_token: &'a str, project: &'a str) -> Logsnag<'a> {
        Logsnag { 
                config: Config::new(api_token, project),
                client: Client::new(),
            }
    }
    pub fn event(&'a self, channel: &'a str, event: &'a str) -> EventBuilder<'_> {
        let event_log = Log::new(&self.config.project, channel, event);

        EventBuilder {
            logsnag: self,
            log: event_log
        }

    }

    pub fn insight<T>(&self, title: &'a str, value: T) -> InsightBuilder<'_>
    where
        T: Into<InsightValue<'static>>,
    {
        let insight_log = Insight::new(&self.config.project, title, value.into());

        InsightBuilder {
            logsnag: self,
            insight: insight_log
        }
    }
}