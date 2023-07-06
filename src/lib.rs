pub mod models;
pub mod client;

use models::{ Log, Insight, InsightValue, Config};
use client::Client;

use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

use anyhow::Error;
use anyhow::anyhow;

//TODO change to v1/ and then add /log or /insights depending on what they want
const PUBLISH_API_URL: &str = "https://api.logsnag.com/v1/log";
const INSIGHT_API_URL: &str = "https://api.logsnag.com/v1/insight";
/// `Logsnag` is a struct used to interact with the Logsnag API.
/// It contains the configuration and client needed to make requests.
#[derive(Clone, Debug)]
pub struct Logsnag {
    pub config: Config,
    pub client: Client,
}

impl Logsnag {

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
    pub fn new(api_token: String, project: String) -> Logsnag {
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
    /// 
    /// // If you don't use env vars, make sure to pass .to_string() on each of the parameters if you initialize the variables inline
    /// let logsnag = Logsnag::new(
    ///    env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key found"), 
    ///    env::var("LOGSNAG_PROJECT").expect("No Logsnag Project found")
    /// );
    /// 
    /// let publish_response = client.publish("my-channel".to_string(), 
    ///     "my-event".to_string(), 
    ///     Some("My Description".to_string()), 
    ///     Some("❤️".to_string()), 
    ///     Some(true))
    ///     .await;
    /// ```
    /// 
    /// Note that non-required values are Options. So you need to wrap them in Some() or use None.
    /// ```
    /// let publish_response = client.publish("my-channel".to_string(), 
    ///     "my-event".to_string(), 
    ///     Some("My Description".to_string()), 
    ///     None, 
    ///     None)
    ///     .await;
    pub async fn publish(&self, channel: String, event: String, description: Option<String>, icon: Option<String>, notify: Option<bool>) -> Result<Response, Error> {

        let log = Log {
            project: self.config.project.clone(),
            channel: channel,
            event: event,
            description: description,
            icon: icon,
            notify: notify
        };

        let request_data = serde_json::to_value(&log)?;

        let request = self
            .client
            .post(PUBLISH_API_URL)
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
    ///     InsightValue::Str("online".to_string()), //InsightValue::Int(69), InsightValue::Bool(false)
    ///     Some("❤️".to_string())) //or None
    ///     .await.expect("Failed to publish insight");
    /// ```
    pub async fn insight(&self, title: String, value: InsightValue, icon: Option<String>) -> Result<Response, Error> {
        let insight = Insight {
            project: self.config.project.clone(),
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