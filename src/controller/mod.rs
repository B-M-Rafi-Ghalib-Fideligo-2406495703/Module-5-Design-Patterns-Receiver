use rocket::fairing::AdHoc;

pub mod notification;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        use notification::subscribe;
        rocket
            .mount("/", routes![subscribe])
    });
}
