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

        let logsnag_key = env::var("LOGSNAG_API_KEY").expect("No Logsnag API Key (LOGSNAG_API_KEY) found in environment variables.");
        let logsnag_project = env::var("LOGSNAG_PROJECT").expect("No Logsnag Project (LOGSNAG_PROJECT) found in environment variables.");

        let logsnag = Logsnag::new(
            &logsnag_key,
            &logsnag_project
        );

        let mut tags = TagHashMap::new();

        tags.insert("guild-id", "test-guild-id");
        tags.insert("User_Name", "test-username-id");

        let publish_result = logsnag.publish(
            "test",
            "Test Event",
            Some("A description here"),
            Some("❤️"),
            Some(true),
            Some(tags),
        ).await;

        let publish_result2 = logsnag.publish(
            "test",
            "Test Event 2",
            Some("A description two here"),
            Some("❤️"),
            Some(true),
            None,
        ).await;

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

        let insight_result = logsnag.insight(
            "Insight Title",
            InsightValue::Int(32),
            Some("❤️"),
        ).await;

        println!("{:?}", insight_result);

        assert!(insight_result.is_ok());
    }
}