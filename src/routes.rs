use actix_web::{middleware::Logger, web};

use crate::handlers as h;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("/live", web::get().to(h::health::ping))
            .route("/ready", web::get().to(h::health::ping)),
    )
    .service(
        web::resource("/")
            .route(web::get().to(h::ip::count_ip))
            .wrap(Logger::default()),
    );
}
