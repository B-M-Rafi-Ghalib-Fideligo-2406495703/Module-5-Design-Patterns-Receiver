use crate::service::notification::NotificationService;
use rocket::serde::json::Json;

#[rocket::get("/subscribe/<product_type>")]
pub async fn subscribe(
    product_type: String,
) -> bambangshop_receiver::Result<Json<crate::model::subscriber::SubscriberRequest>> {
    let data = NotificationService::subscribe(product_type).await?;
    Ok(Json(data))
}

#[rocket::get("/unsubscribe/<product_type>")]
pub async fn unsubscribe(
    product_type: String,
) -> bambangshop_receiver::Result<Json<crate::model::subscriber::SubscriberRequest>> {
    let data = NotificationService::unsubscribe(product_type).await?;
    Ok(Json(data))
}

