use crate::models::User;
use actix_web::dev::ServiceRequest;
use actix_web::{middleware, web, App, Error, HttpServer,ResponseError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::headers::www_authenticate::basic::Basic;
use actix_web_httpauth::extractors::AuthenticationError;
use sqlx::{Pool, SqliteConnection};

pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let app_data: web::Data<Pool<SqliteConnection>> = req.app_data().unwrap();
    let username = credentials.user_id().to_string();
    if let Some(pass) = credentials.password() {
        if let Ok(user) = User::find_by_username(&app_data, username).await {
            if user.password == *pass {
               return Ok(req);
            }
        }
    }
    Err(Error::from(AuthenticationError::new(Basic::with_realm("Restricted area"))))
}
