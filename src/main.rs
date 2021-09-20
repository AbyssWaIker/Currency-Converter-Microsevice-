mod conversion_rates;
mod config;
use std::{
    thread,
    time::{Duration},
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, error, http};
use actix_cors::Cors;
use crate::conversion_rates::{ErrorResponse, ConversionRequest, ExchangeRates, update_rates_toml, CurrenciesResponse, CurrencyType};
use crate::config::MyConfig;
use confy::ConfyError;

const CONFIG_PATH: &str = "config.toml";
const EXCHANGE_RATES_PATH: &str = "exchange_rates.toml";

#[get("/")]
async fn get_all_currencies() -> HttpResponse {

    let exchange_rates: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);

    if exchange_rates.is_err() {
        return HttpResponse::Ok()
            .header("Access-Control-Allow-Origin","[*]")
            .json(ErrorResponse::construct("failed to load exchange rates"))

    }

    let currencies: Box<[CurrencyType]> = exchange_rates.unwrap().rates.into_keys().collect();
    HttpResponse::Ok().json(CurrenciesResponse::construct(currencies))
}

#[post("/convert")]
async fn convert(request: web::Json<ConversionRequest>) -> HttpResponse {

    let exchange_rates: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);
    if exchange_rates.is_err() {
        return HttpResponse::Ok()
            .json(ErrorResponse::construct("failed to load exchange rates"))
    }
    let result = exchange_rates.unwrap().convert(request.into_inner());
    let response = match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(error) => HttpResponse::Ok().json(error),
    };

    response
}

async fn update_rates_every_hour() {
    let conf : MyConfig = confy::load_path(CONFIG_PATH).unwrap();
    loop {
        thread::sleep(Duration::from_secs(60 * 60));
        update_rates_toml(&conf.api_key,EXCHANGE_RATES_PATH);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf : MyConfig = confy::load_path(CONFIG_PATH).unwrap();

    update_rates_toml(&conf.api_key,EXCHANGE_RATES_PATH);
    let er: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);
    if er.is_err() || er.unwrap().rates.len() == 0 {
        panic!("Failed to fetch exchange rates.")
    }

    let _new_thread = thread::spawn(update_rates_every_hour);

    let url = format!("{}:{}",conf.host,conf.port);
    println!("server is starting at http://{}", url);
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_all_currencies)
            .service(convert)
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                          error::InternalError::from_response(
                                  "",
                                  HttpResponse::BadRequest()
                                          .content_type("application/json")
                                          .body(format!(r#"{{"error":"{:?}"}}"#, err)),
                              )
                              .into()
                          }))
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(url)?
    .run()
    .await
}
