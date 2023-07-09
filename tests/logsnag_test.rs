#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use logsnag::Logsnag;

    #[tokio::test]
    async fn test_publish() {
        dotenv().ok();

        let logsnag_key = env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in environment variables.");
        let logsnag_project = env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in environment variables.");

        let logsnag = Logsnag::new(
            &logsnag_key,
            &logsnag_project
        );

        //channel name must be a string of lowercase letterse, numbers, underscores, and dashes
        let publish_result = logsnag.event("test","Test Event")
            .with_notify(true)
            .with_description("ayooooo")
            .with_icon("💀")
            .with_tag("tAg-one", "tag88-value")
            .with_tag("tag_another", "tag-2-value")
            .publish()
            .await;

        println!("{:?}", publish_result);

        assert!(publish_result.is_ok());
    }

    #[tokio::test]
    async fn test_insight() {
        dotenv().ok();

        let logsnag_key = env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in environment variables.");
        let logsnag_project = env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in environment variables.");

        let logsnag = Logsnag::new(
            &logsnag_key,
            &logsnag_project);

        let insight_result = logsnag.insight("Title", "hello")
            .with_icon("💀")
            .publish()
            .await;

        println!("{:?}", insight_result);

        assert!(insight_result.is_ok());
    }
}