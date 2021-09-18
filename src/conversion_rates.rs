use curl::easy::Easy;
use std::collections::hash_map::Keys;
use std::ops::Not;

type Currency = f64;
type CurrencyType = [char; 3];
type ExchangeRate = f64;
type Rates = std::collections::hash_map::HashMap<CurrencyType, ExchangeRate>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRates {
    base: CurrencyType,
    rates: Rates,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversionRequest {
    current: CurrencyType,
    target: CurrencyType,
    sum: Currency,
}

#[derive(Serialize)]
pub struct SuccessfulResponse {
    status: bool,
    sum: Currency,
}
#[derive(Serialize)]
pub struct ErrorResponse {
    status: bool,
    error: String,
}

pub  type Response = std::result::Result<SuccessfulResponse, ErrorResponse>;

impl SuccessfulResponse {
    fn construct(sum: f64) -> SuccessfulResponse {
        SuccessfulResponse { status: true, sum }
    }
}

impl ErrorResponse {
    fn construct(error: String) -> ErrorResponse {
        ErrorResponse {
            status: false,
            error,
        }
    }
}

impl ExchangeRates {
    pub fn update(&mut self) {
        let url = "https://openexchangerates.org/api/latest.json";
        let api_key = "284f367e62cb435d96cdfff3f86b6490";
        let final_url = format!("{}?app_id={}", url, api_key);

        let mut easy = Easy::new();
        easy.url(final_url.as_str()).unwrap();
        easy.write_function(|data| {
            er: ExchangeRates = serde_json::from_slice(data).unwrap();
            self.rates = er.rates.clone();
            self.base = er.base.clone();
            Ok(data.len())
        })
        .unwrap();
        easy.perform().unwrap();
    }
    pub fn get_all_currencies(&self) -> Keys<'_, CurrencyType, ExchangeRate> {
        self.rates.keys().clone()
    }
    pub fn check_if_currency_exists(&self, c: CurrencyType) -> bool {
        self.base.eq(&c) || self.rates.contains_key(&c)
    }

    fn error(msg: String, obj: String) -> Response::Error {
        let error = format!("{} {}", msg, obj);
        return Response::Error(ErrorResponse::construct(error));
    }
    fn success(sum: Currency) -> Response::Success {
        return Response::Success(SuccessfulResponse::construct(sum));
    }

    pub fn convert(&self, r: ConversionRequest) -> Response {
        if self.check_if_currency_exists(r.current).not() {
            let error_msg = "Wrong currency provided:";
            return self.error(error_msg, r.current);
        }
        if self.check_if_currency_exists(r.target).not() {
            let error_msg = "Wrong currency provided:";
            return self.error(error_msg, r.target);
        }
        if self.rates.keys().count().eq(&0) {
            let error_msg = "Wrong currency provided";
            return self.error(error_msg, String(""));
        }

        if r.target == r.current {
            return self.success(r.sum);
        }

        if self.base.eq(&r.current) {
            let result = self.convert_from_base(r.sum, r.target);
            return self.success(result);
        }

        if self.base.eq(&r.target) {
            let result = self.convert_to_base(r.sum, r.current);
            return self.success(result);
        }

        let sum_in_base_currency = self.convert_to_base(r.sum, r.current);
        let result = self.convert_from_base(sum_in_base_currency, r.target);

        self.success(result)
    }

    fn convert_from_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum * self.rates.get(&currency)
    }
    fn convert_to_base(&self, sum: Currency, currency: CurrencyType) -> f64 {
        sum / self.rates.get(&currency)
    }
}
