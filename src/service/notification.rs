pub struct NotificationService;

impl NotificationService {
    pub async fn subscribe(
        product_type: String,
    ) -> bambangshop_receiver::Result<crate::model::subscriber::SubscriberRequest> {
        use bambangshop_receiver::{compose_error_response, APP_CONFIG, REQWEST_CLIENT};
        use rocket::http::Status;

        let url = format!(
            "{}/notification/subscribe/{}",
            APP_CONFIG.get_publisher_root_url(),
            product_type
        );

        let subscriber_request = crate::model::subscriber::SubscriberRequest {
            url: APP_CONFIG.get_instance_root_url().to_string(),
            name: APP_CONFIG.get_instance_name().to_string(),
        };

        let response: std::result::Result<reqwest::Response, reqwest::Error> = REQWEST_CLIENT
            .post(&url)
            .json(&subscriber_request)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => Ok(subscriber_request),
            Ok(_) => Err(compose_error_response(
                Status::BadRequest,
                format!("Publisher rejected subscribe for {}", product_type),
            )),
            Err(_) => Err(compose_error_response(
                Status::NotFound,
                format!("Failed to reach publisher for {}", product_type),
            )),
        }
    }
}

