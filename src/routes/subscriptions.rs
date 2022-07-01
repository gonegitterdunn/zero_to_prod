use actix_web::{web, HttpResponse};

use crate::FormData;

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
