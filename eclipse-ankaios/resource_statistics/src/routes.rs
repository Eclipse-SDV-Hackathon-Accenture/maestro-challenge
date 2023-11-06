use crate::resource_statistic_job::ItemCache;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn statistics(cache: web::Data<ItemCache>) -> impl Responder {
    match cache.lock() {
        Ok(entry) => HttpResponse::Ok().json(&*entry),
        Err(_) => HttpResponse::InternalServerError()
            .body("failed to request the resource statistics on the host."),
    }
}
