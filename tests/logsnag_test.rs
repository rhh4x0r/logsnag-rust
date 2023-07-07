#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;
    use std::env;

    use logsnag::Logsnag;
    use logsnag::models::{ InsightValue, TagHashMap };

    #[tokio::test]
    async fn test_publish() {

        dotenv().ok();

        let logsnag = Logsnag::new(
            env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in environment variables."),
            env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in environment variables.")
        );

        let mut tags = TagHashMap::new();

        tags.insert("guild-id", "test-guild-id");
        tags.insert("User_Name", "test-username-id");

        let publish_result = logsnag.publish(
            "test".to_string(),
            "Test Event".to_string(),
            Some("A description here".to_string()),
            Some("❤️".to_string()),
            Some(true),
            Some(tags),
        ).await;

        println!("{:?}", publish_result);

        assert!(publish_result.is_ok());

    }

    #[tokio::test]
    async fn test_insight() {
        let logsnag = Logsnag::new(
            "".to_string(),
            "".to_string()
        );

        let insight_result = logsnag.insight(
            "Insight Title".to_string(),
            InsightValue::Int(32),
            Some("❤️".to_string()),
        ).await;

        println!("{:?}", insight_result);

        assert!(insight_result.is_ok());
    }
}