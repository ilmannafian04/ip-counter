use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::{services, states::IpCounter};

pub async fn count_ip(ip_counter: web::Data<IpCounter>, req: HttpRequest) -> impl Responder {
    let request_ip = match req.connection_info().realip_remote_addr() {
        Some(ip) => ip.to_string(),
        None => match req.peer_addr() {
            Some(ip) => ip.to_string(),
            None => return HttpResponse::BadRequest().finish(),
        },
    };

    let mut ip_map = match ip_counter.maps.lock() {
        Ok(lock) => lock,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match services::ip::increment(&mut ip_map, &request_ip) {
        Ok(hits) => HttpResponse::Ok().body(format!(
            "User address: {}\nHits: {}",
            &req.peer_addr().unwrap().ip().to_string(),
            hits
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
