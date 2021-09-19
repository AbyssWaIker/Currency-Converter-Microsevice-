use curl::easy::{Easy, WriteError};
use std::collections::hash_map::Keys;
use std::ops::Not;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::iter::FromIterator;
use curl::Error;

pub type Currency = f64;
pub type CurrencyType = String;
pub type ExchangeRate = f64;
pub type Rates = std::collections::hash_map::HashMap<CurrencyType, ExchangeRate>;


#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRates {
    pub base: CurrencyType,
    pub rates: Rates,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversionRequest {
    pub current: CurrencyType,
    pub target: CurrencyType,
    pub sum: Currency,
}

#[derive(Serialize)]
pub struct SuccessfulResponse {
    pub status: bool,
    pub sum: Currency,
}


#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: bool,
    pub error: String,
}

pub type Response = std::result::Result<SuccessfulResponse, ErrorResponse>;

impl SuccessfulResponse {
    pub fn construct(sum: f64) -> SuccessfulResponse {
        SuccessfulResponse { status: true, sum }
    }
}

#[derive(Serialize)]
pub struct CurrenciesResponse {
    pub status: bool,
    pub data: Box<[CurrencyType]>,
}
impl CurrenciesResponse {
    pub fn construct(data: Box<[CurrencyType]>) -> CurrenciesResponse {
        CurrenciesResponse { status: true, data }
    }
}


impl ErrorResponse {
    pub fn construct(error: String) -> ErrorResponse {
        ErrorResponse {
            status: false,
            error,
        }
    }
}
pub fn update_rates_toml(api_key: &str, path_to_toml:  & str) {
    let url = "https://openexchangerates.org/api/latest.json";
    let final_url = format!("{}?app_id={}", url, api_key);
    let path_to_toml_for_closure = path_to_toml.to_string().clone();

    let mut easy = Easy::new();
    easy.url(&final_url).unwrap();
    easy.write_function(move |data| {
        let a = String::from_utf8(data.to_vec());
        let er: serde_json::error::Result<ExchangeRates> = serde_json::from_slice(data);
        if er.is_err() {
            panic!(er);
            return Ok(0)
        }
        confy::store_path(path_to_toml_for_closure.to_owned(), er.unwrap()).unwrap();
        Ok(data.len())
    })
        .unwrap();
    easy.perform().unwrap()
}
impl ::std::default::Default for ExchangeRates {
    fn default() -> Self { Self { base: "USD".to_string(), rates: Rates::new() } }
}
impl ExchangeRates {
    pub fn get_all_currencies(&self) -> Keys<'_, CurrencyType, ExchangeRate> {
        self.rates.keys().clone()
    }
    pub fn check_if_currency_exists(&self, c: CurrencyType) -> bool {
        self.base.eq(&c) || self.rates.contains_key(&c)
    }

    pub fn convert(&self, r: ConversionRequest) -> Response {
        if self.check_if_currency_exists(r.current.clone()).not() {
            let error = format!("Wrong currency provided {}", r.current);
            return Response::Err(ErrorResponse::construct(error));
        }
        if self.check_if_currency_exists(r.target.clone()).not() {
            let error = format!("Wrong currency provided {}", r.target);
            return Response::Err(ErrorResponse::construct(error));
        }
        if self.rates.keys().count().eq(&0) {
            let error_msg = "Wrong currency provided";
            return Response::Err(ErrorResponse::construct(error_msg.to_string()));
        }

        if r.target == r.current {
            return Response::Ok(SuccessfulResponse::construct(r.sum));
        }

        if self.base.eq(&r.current) {
            let result = self.convert_from_base(r.sum, r.target);
            return Response::Ok(SuccessfulResponse::construct(result));
        }

        if self.base.eq(&r.target) {
            let result = self.convert_to_base(r.sum, r.current);
            return Response::Ok(SuccessfulResponse::construct(result));
        }

        let sum_in_base_currency = self.convert_to_base(r.sum, r.current);
        let result = self.convert_from_base(sum_in_base_currency, r.target);

        return Response::Ok(SuccessfulResponse::construct(result));
    }

    fn convert_from_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum * self.rates.get(&currency).unwrap()
    }
    fn convert_to_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum / self.rates.get(&currency).unwrap()
    }
}
