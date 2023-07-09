#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use logsnag::Logsnag;

    #[tokio::test]
    async fn test_publish() {
        dotenv().ok();

        let logsnag = Logsnag::new(
            env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in Environment."),
            env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in Environment.")
        );

        //channel name must be a string of lowercase letters, numbers, underscores, and dashes
        let publish_result = logsnag.event("test","Test Event")
            .with_notify(true)
            .with_description("This is a test description.")
            .with_icon("🥳")
            .with_tag("tAg-one", "tag-value")
            .with_tag("tagtwo", "tag-2-value")
            .publish()
            .await;

        let publish_result = logsnag.event("test","Test Event 2")
            .with_notify(true)
            .with_description("This is a test description.")
            .with_icon("🥳")
            .with_tag("tAg-one", "tag-value")
            .with_tag("tagtwo", "tag-2-value")
            .publish()
            .await;

        println!("{:?}", publish_result);

        assert!(publish_result.is_ok());
    }

    #[tokio::test]
    async fn test_insight() {
        dotenv().ok();

        let logsnag = Logsnag::new(
            env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in Environment."),
            env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in Environment.")
        );

        let insight_result = logsnag.insight("status", "online")
            .with_icon("🟢")
            .publish()
            .await;

        println!("{:?}", insight_result);

        assert!(insight_result.is_ok());
    }
}