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

#[rocket::post("/receive", data = "<data>")]
pub async fn receive(
    data: Json<crate::model::notification::Notification>,
) -> bambangshop_receiver::Result<Json<crate::model::notification::Notification>> {
    let notification = data.into_inner();
    let saved = NotificationService::receive_notification(notification)?;
    Ok(Json(saved))
}

#[rocket::get("/")]
pub fn list() -> bambangshop_receiver::Result<Json<Vec<String>>> {
    let messages = NotificationService::list_messages()?;
    Ok(Json(messages))
}

