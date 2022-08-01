use serde_json::{Value};

const COIN_GECKO_BASE_URL: &str = "https://api.coingecko.com/api/v3/";

fn request_coin_gecko_price(coin_one_id: &str, coin_two_id: &str) -> Result<String, reqwest::Error> {
    let url: String = format!("{}simple/price?ids={}&vs_currencies={}", COIN_GECKO_BASE_URL, coin_one_id, coin_two_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}

fn get_coin_gecko_price(coin_one_id: &str, coin_two_id: &str) -> Option<f64> {
    let resp: Result<String, reqwest::Error> = request_coin_gecko_price(coin_one_id, coin_two_id);
    if resp.is_err() {
        return None;
    } else {
        let resp_json: Value = serde_json::from_str(&resp.unwrap()).unwrap();
        let price: f64 = resp_json[coin_one_id][coin_two_id].as_f64().unwrap();
        return Some(price);
    }
}

pub fn get_price(coin_one_id: &str, coin_two_id: &str) -> Option<f64> {
    let coin_one_price: Option<f64> = get_coin_gecko_price(coin_one_id, "usd");
    if coin_one_price.is_none() {
        return None;
    }
    let coin_two_price: Option<f64> = get_coin_gecko_price(coin_two_id, "usd");
    if coin_two_price.is_none() {
        return None;
    }
    let coin_one_price: f64 = coin_one_price.unwrap();
    let coin_two_price: f64 = coin_two_price.unwrap();
    return Some(coin_one_price / coin_two_price);
}