mod conversion_rates;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{stdout, Write};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let sum = req_body.parse::<f32>();
    if sum.is_err() {
        return HttpResponse::Ok().json(ErrorResponse {
            status: false,
            error: "failed to parse sum".parse().unwrap(),
        });
    }
    HttpResponse::Ok().json(SuccessfulResponse {
        status: true,
        sum: sum.unwrap() * 2.0,
    })
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
