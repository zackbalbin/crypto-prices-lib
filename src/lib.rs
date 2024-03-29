use serde_json::{Value};

/// The Coin Gecko API base url.
pub const COIN_GECKO_BASE_URL: &str = "https://api.coingecko.com/api/v3/";

/// Calls the Coin Gecko API and returns the price as a String. 
fn request_coin_gecko_price(coin_one_id: &str, coin_two_id: &str) -> Result<String, reqwest::Error> {
    let url: String = format!("{}simple/price?ids={}&vs_currencies={}", COIN_GECKO_BASE_URL, coin_one_id, coin_two_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}

/// Calls the Coin Gecko API and returns the marketcap as a String.
fn request_coin_gecko_marketcap(coin_one_id: &str, coin_two_id: &str) -> Result<String, reqwest::Error> {
    let url: String = format!("{}simple/price?ids={}&vs_currencies={}&include_market_cap=true", COIN_GECKO_BASE_URL, coin_one_id, coin_two_id);
    let resp: String = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}

/// Parses the Coin Gecko API response and returns the price as a f64.
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

/// Parses the Coin Gecko API response and returns the marketcap as a f64.
fn get_coin_gecko_marketcap(coin_one_id: &str, coin_two_id: &str) -> Option<f64> {
    let resp: Result<String, reqwest::Error> = request_coin_gecko_marketcap(coin_one_id, coin_two_id);
    if resp.is_err() {
        return None;
    } else {
        let resp_json: Value = serde_json::from_str(&resp.unwrap()).unwrap();
        let key: String = format!("{}_market_cap", coin_two_id);
        let marketcap: f64 = resp_json[coin_one_id][key].as_f64().unwrap();
        return Some(marketcap);
    }
}

/// Returns the price of a coin against another coin.
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

/// Returns the marketcap of a coin against another coin.
pub fn get_marketcap(coin_one_id: &str, coin_two_id: &str) -> Option<f64> {
    let coin_one_marketcap: Option<f64> = get_coin_gecko_marketcap(coin_one_id, "usd");
    if coin_one_marketcap.is_none() {
        return None;
    }
    let coin_two_price: Option<f64> = get_coin_gecko_price(coin_two_id, "usd");
    if coin_two_price.is_none() {
        return None;
    }
    let coin_one_marketcap: f64 = coin_one_marketcap.unwrap();
    let coin_two_price: f64 = coin_two_price.unwrap();
    return Some(coin_one_marketcap / coin_two_price);
}