mod conversion_rates;
mod config;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::conversion_rates::{SuccessfulResponse, ErrorResponse, ConversionRequest, ExchangeRates, update_rates_toml, CurrenciesResponse};
use crate::config::MyConfig;
use confy::ConfyError;

const CONFIG_PATH: &str = "config.toml";
const EXCHANGE_RATES_PATH: &str = "exchange_rates.toml";

fn error(msg: &str) -> HttpResponse {
    HttpResponse::Ok()
        .json(ErrorResponse::construct(msg.to_string()))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(currency: String) -> HttpResponse {

    let exchange_rates: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);
    if exchange_rates.is_err() {
        return error("failed to load exchange rates")
    }
    let exchange_rates: ExchangeRates = exchange_rates.unwrap();
    let result = exchange_rates.rates.get(&currency);
    if result.is_none() {
        return error("Currency isn't found")
    }
    let result = *result.unwrap();

    return HttpResponse::Ok().json(SuccessfulResponse::construct(result))
}

#[get("/all_currencies")]
async fn get_all_currencies() -> HttpResponse {

    let exchange_rates: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);
    if exchange_rates.is_err() {
        return error("failed to load exchange rates")
    }
    return HttpResponse::Ok().json(CurrenciesResponse::construct(exchange_rates.unwrap().rates.into_keys().collect()))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf : MyConfig = confy::load_path(CONFIG_PATH).unwrap();

    update_rates_toml(&conf.api_key,EXCHANGE_RATES_PATH);
    let er: Result<ExchangeRates, ConfyError> = confy::load_path(EXCHANGE_RATES_PATH);
    if er.is_err() || er.unwrap().rates.len() == 0 {
        panic!("Failed to fetch exchange rates.")
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_all_currencies)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
