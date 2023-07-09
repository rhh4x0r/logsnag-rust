pub mod models;
pub mod client;

use std::collections::HashMap;

use models::{ Log, Insight, InsightValue, TagHashMap, Config};
use client::Client;

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

        println!("{:?}", request_data);

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
    /// let client = Logsnag::new("my-api-token".to_string(), "my-project".to_string());
    /// ```
    pub fn new(api_token: &'a str, project: &'a str) -> Logsnag<'a> {
        Logsnag { 
                config: Config::new(api_token, project),
                client: Client::new(),
            }
    }
    /// Publishes a log to the given channel with the specified event and optional description, icon, and notify flag.
    ///
    /// # Arguments
    ///
    /// * `&self` - The instance of the `Logsnag` struct.
    /// * `channel` - A `String` representing the channel to which the log is to be published.
    /// * `event` - A `String` representing the event that is to be logged.
    /// * `description` - An `Option<String>` that can contain the description of the event. This is optional.
    /// * `icon` - An `Option<String>` that can contain the URI of the icon to be used with the log. This is optional.
    /// * `notify` - An `Option<bool>` that can flag whether or not to notify the channel of the event. This is optional.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - Returns a `Result` type. On success, it contains a `Response` which represents the server's response to the request (`reqwest::async_impl::response`). On failure, it contains an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the server's response indicates a failure (i.e., the response status is not 200).
    /// # Examples
    ///
    /// ```
    /// use logsnag::Logsnag;
    /// use Logsnag::models::TagHashMap;
    /// 
    /// // If you don't use env vars, make sure to pass .to_string() on each of the parameters if you initialize the variables inline
    /// let logsnag = Logsnag::new(
    ///    env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key found"), 
    ///    env::var("LOGSNAG_PROJECT").expect("No Logsnag Project found")
    /// );
    /// 
    /// let mut tags = TagHashMap::new();
    /// tags.insert("guild-id", "test-guild-id");
    /// tags.insert("User_Name", "test-username-id"); //will auto lowercase and change "_" to "-" to fit API constraints
    /// 
    /// let publish_response = client.publish("my-channel".to_string(), 
    ///     "my-event".to_string(), 
    ///     Some("My Description".to_string()), 
    ///     Some("❤️".to_string()), 
    ///     Some(true),
    ///     Some(tags))
    ///     .await;
    /// ```
    /// 
    /// Note that non-required values are Options. So you need to wrap them in Some() or use None.
    /// ```
    /// let publish_response = client.publish("my-channel".to_string(), 
    ///     "my-event".to_string(), 
    ///     Some("My Description".to_string()), 
    ///     None, 
    ///     None,
    ///     None)
    ///     .await;
    pub fn event(&'a self, channel: &'a str, event: &'a str) -> EventBuilder<'_> {
        let event_log = Log::new(&self.config.project, channel, event);

        EventBuilder {
            logsnag: self,
            log: event_log
        }

    }

    /// Publishes an insight with the given title, event, value, and an optional icon.
    ///
    /// # Arguments
    ///
    /// * `&self` - The instance of the `Logsnag` struct.
    /// * `title` - A `String` representing the title of the insight.
    /// * `event` - A `String` representing the event related to the insight.
    /// * `value` - An `InsightValue` that can be either a `String` or a numeric type, depending on the specific insight. For example, for an online status insight, you might use `InsightValue::new("online")`, and for a numeric count of errors, you might use `InsightValue::new(10)`.
    /// * `icon` - An `Option<String>` that can contain the icon to be used with the insight. This is optional.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - Returns a `Result` type. On success, it contains a `Response` which represents the server's response to the request. On failure, it contains an `Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use logsnag::Logsnag;
    /// use logsnag::models::InsightValue;
    ///
    /// let client = Logsnag::new("my-api-token".to_string(), "my-project".to_string());
    /// let response = client.insight("my-title".to_string(), 
    ///     "my-event".to_string(), 
    ///     InsightValue::Str("online"), //InsightValue::Int(69), InsightValue::Bool(false)
    ///     Some("❤️".to_string())) //or None
    ///     .await.expect("Failed to publish insight");
    /// ```
    pub async fn insight(&self, title: &str, value: InsightValue, icon: Option<&str>) -> Result<Response, Error> {
        let insight = Insight {
            project: self.config.project,
            title: title,
            value: value,
            icon: icon
        };

        let request_data = serde_json::to_value(&insight)?;

        let request = self
            .client
            .post(INSIGHT_API_URL)
            .json(&request_data)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.config.api_token);

        let response = request.send().await?;

        if response.status() == 200 {
            Ok(response)
        } else {
            Err(anyhow!("Error in response: {:?}", response.text().await))
        }
    }
}