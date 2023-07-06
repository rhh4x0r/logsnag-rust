mod models;
mod client;

use models::{ Logsnag, Log, Insight, InsightValue, Config};
use client::Client;

use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

use anyhow::Error;
use anyhow::anyhow;

//TODO change to v1/ and then add /log or /insights depending on what they want
const PUBLISH_API_URL: &str = "https://api.logsnag.com/v1/log";
const INSIGHT_API_URL: &str = "https://api.logsnag.com/v1/insight";

impl Logsnag {
    pub fn new(api_token: &str, project: &str) -> Logsnag {
        Logsnag { 
                config: Config::new(api_token, project),
                client: Client::new(),
            }
    }

    pub async fn publish(self, channel: String, event: String, description: Option<String>, icon: Option<String>, notify: Option<bool>) -> Result<Response, Error> {

        let log = Log {
            project: self.config.project,
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
            println!("Sent log to client.");

            Ok(response)
        } else {
            Err(anyhow!("Error in response: {:?}", response.text().await))
        }
    }

    pub async fn insight(self, title: String, value: InsightValue, icon: Option<String>) -> Result<Response, Error> {
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