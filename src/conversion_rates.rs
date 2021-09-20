use curl::easy::{Easy};
use std::ops::Not;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type Currency = f64;
pub type CurrencyType = String;
pub type ExchangeRate = f64;
pub type Rates = std::collections::hash_map::HashMap<CurrencyType, ExchangeRate>;


#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRates {
    pub base: CurrencyType,
    pub rates: Rates,
}

#[derive(Debug, Deserialize)]
pub struct ConversionRequest {
    pub from: CurrencyType,
    pub to: CurrencyType,
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
    pub fn construct(error: &str) -> ErrorResponse {
        ErrorResponse {
            status: false,
            error: error.to_string(),
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
        // let a = String::from_utf8(data.to_vec());
        let er: serde_json::error::Result<ExchangeRates> = serde_json::from_slice(data);
        if er.is_err() {
            panic!("{:?}",er);
            // return Ok(0)
        }
        let mut er = er.unwrap();
        er.rates.insert(er.base.clone(), 1.0);
        confy::store_path(path_to_toml_for_closure.to_owned(), er).unwrap();
        Ok(data.len())
    })
        .unwrap();
    easy.perform().unwrap()
}
impl ::std::default::Default for ExchangeRates {
    fn default() -> Self { Self { base: "USD".to_string(), rates: Rates::new() } }
}
impl ExchangeRates {
    // pub fn get_all_currencies(&self) -> Box<[CurrencyType]> {
    //     self.rates.into_keys().collect()
    // }
    pub fn check_if_currency_exists(&self, c: CurrencyType) -> bool {
        self.base.eq(&c) || self.rates.contains_key(&c)
    }

    pub fn convert(&self, r: ConversionRequest) -> Response {
        if self.check_if_currency_exists(r.from.clone()).not() {
            let error = format!("Wrong currency provided {}", r.from);
            return Response::Err(ErrorResponse::construct(error.as_str()));
        }
        if self.check_if_currency_exists(r.to.clone()).not() {
            let error = format!("Wrong currency provided {}", r.to);
            return Response::Err(ErrorResponse::construct(error.as_str()));
        }
        if self.rates.keys().count().eq(&0) {
            let error_msg = "Wrong currency provided";
            return Response::Err(ErrorResponse::construct(error_msg));
        }

        if r.to == r.from {
            return Response::Ok(SuccessfulResponse::construct(r.sum));
        }

        if self.base.eq(&r.from) {
            let result = self.convert_from_base(r.sum, r.to);
            return Response::Ok(SuccessfulResponse::construct(result));
        }

        if self.base.eq(&r.to) {
            let result = self.convert_to_base(r.sum, r.from);
            return Response::Ok(SuccessfulResponse::construct(result));
        }

        let sum_in_base_currency = self.convert_to_base(r.sum, r.from);
        let result = self.convert_from_base(sum_in_base_currency, r.to);

        return Response::Ok(SuccessfulResponse::construct(result));
    }

    fn convert_from_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum * self.rates.get(&currency).unwrap()
    }
    fn convert_to_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum / self.rates.get(&currency).unwrap()
    }
}
