use actix_web::{Responder};

#[get("/api/v1/captcha/ping")]
pub async fn ping() -> impl Responder{
    "Pong \n"
}

#[post("/api/v1/captcha/new")]
pub async fn new_captcha() -> impl Responder{
    "Ok"
}