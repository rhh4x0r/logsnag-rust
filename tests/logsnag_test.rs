#[cfg(test)]
mod tests {
    use super::*;

    use logsnag::Logsnag;
    use logsnag::models::InsightValue;

    #[tokio::test]
    async fn test_publish() {
        //TODO: find a way to use ENV variables in tests easily
        let logsnag = Logsnag::new(
            "".to_string(),
            "".to_string()
        );

        let publish_result = logsnag.publish(
            "test".to_string(),
            "Test Event".to_string(),
            Some("A description here".to_string()),
            Some("❤️".to_string()),
            Some(true)
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