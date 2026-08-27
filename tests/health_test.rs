#[cfg(test)]
mod tests {
    use troit_logistics_backend::routes::health_handler;

    #[tokio::test]
    async fn test_health_check_status_ok() {
        let response = health_handler().await;
        assert_eq!(response.0["status"], "ok");
        assert_eq!(response.0["service"], "TROIT Logistics Backend API");
    }
}
