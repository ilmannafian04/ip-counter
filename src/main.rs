use std::{collections::HashMap, sync::Mutex};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

struct IpCounter {
    pub maps: Mutex<HashMap<String, usize>>,
}

async fn hello(state: web::Data<IpCounter>, req: HttpRequest) -> impl Responder {
    let request_ip = req.peer_addr().unwrap().ip().to_string();
    let mut counter = state.maps.lock().unwrap();
    let hits: usize;
    if counter.contains_key(&request_ip) {
        let current_count = counter.get(&request_ip).unwrap().clone() + 1;
        hits = current_count;
        counter.insert(request_ip, current_count);
    } else {
        hits = 1;
        counter.insert(request_ip, 1);
    }
    HttpResponse::Ok().body(format!(
        "User address: {}\nHits: {}",
        &req.peer_addr().unwrap().ip().to_string(),
        hits
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(IpCounter {
        maps: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(hello))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
