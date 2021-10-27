/*
Kraken's API:
    https://api.kraken.com/0/public/Ticker?pair=BTCUSD

Coinbase's API:
    https://api.coinbase.com/v2/prices/BTC-USD/buy
    https://api.coinbase.com/v2/prices/ETH-USD/sell

This program will write to prices.json the following:
    "kraken": {
        "BTC": {
            "buy": x,
            "sell": x
        },
        "ETH": {
            "buy": x,
            "sell": x
        }
    },
    "coinbase": {
        "BTC": {
            "buy": x,
            "sell": x
        },
        "ETH": {
            "buy": x,
            "sell": x
        }
    }
 * */

use reqwest;
use actix_web::{get, http, HttpServer, HttpResponse, web, App, Responder};
use serde::{self, Serialize, Deserialize};
use serde_json::{self, Value};
use std::{time::Duration, sync::RwLock, thread};


#[derive (Debug)]
struct Currency {
    symbol: String,
    buy: f32, // TODO use money lib
    sell: f32, // TODO use money lib
}

impl Currency {
    fn new(symbol: &str) -> Self {
        Currency{
            symbol: symbol.to_string(),
            buy: 0.0,
            sell: 0.0,
        }
    }
}


// TODO Use money lib
#[derive (Serialize, Deserialize, Debug)]
struct JsonCurrency {
    buy: f32,
    sell: f32,
}

#[derive (Serialize, Deserialize, Debug)]
struct JsonExchange {
    name: String,
    BTC: JsonCurrency,
    ETH: JsonCurrency,
}

#[derive (Serialize, Deserialize, Debug)]
struct JsonResponse {
    data: Vec<JsonExchange>,
}

#[derive (Debug)]
struct Exchange {
    name: String, // exchange name
    currencies: Vec<Currency>
}

impl Exchange {
    fn new(name: &str) -> Self {
        let mut currencies: Vec<Currency> = Vec::with_capacity(2);
        currencies.push(Currency::new("BTC"));
        currencies.push(Currency::new("ETH"));

        Exchange {
            name: name.to_string(),
            currencies
        }
    }

    fn to_json(&self) -> JsonExchange {
        let BTC = JsonCurrency { buy: self.currencies[0].buy, sell: self.currencies[0].sell };
        let ETH = JsonCurrency { buy: self.currencies[1].buy, sell: self.currencies[1].sell };

        JsonExchange {name: self.name.clone(), BTC, ETH}
    }
}

#[derive (Deserialize, Debug)]
struct CoinbaseData {
    base: String,
    currency: String,
    amount: String, // TODO convert this to currency 2 decimal places!
}

#[derive (Deserialize, Debug)]
struct CoinbaseResponse {
    data: CoinbaseData,
}


// async fn call_coinbase(coinbase: &mut Exchange) {
fn call_coinbase(coinbase: &mut Exchange) {
    let fiat = "USD";
    // let client = Client::default();

    let actions = ["buy", "sell"];

    for currency in &mut coinbase.currencies {
        for action in &actions {
            let pair = format!["{}-{}", currency.symbol, fiat];
            let url = format!["https://api.coinbase.com/v2/prices/{}/{}", pair, action];

            // Send a BLOCKING request.
            // Can't do async because it won't impl Send,
            // this makes the rust compiler angry.
            match reqwest::blocking::get(url) {
                Ok(data) => {
                    let json: Result<CoinbaseResponse, _> = data.json();
                    match json {
                        Ok(json_response) => {
                            let price = json_response.data.amount.parse::<f32>().unwrap();
                            match *action {
                                "buy" => currency.buy = price,
                                "sell" => currency.sell = price,
                                _ => panic!["We shouldn't have any other action cases!"]
                            }
                        },
                        Err(e) =>  {
                            // TODO: What do we do if we can't get the data?
                            eprintln!("{:?}", e);
                        }
                    }
                },
                Err(e) =>  {
                    // TODO: What do we do if we can't get the data?
                    eprintln!("{:?}", e);
                }
            }
        }
    }
}

fn call_kraken(kraken: &mut Exchange) {
    let fiat = "USD";

    for currency in &mut kraken.currencies {
        // Generate the url for each currency.
        // Ideally, we would produce these once and reuse them,
        // but it would be tricky to access them in a loop.
        let pair = format!["{}{}", currency.symbol, fiat];
        let url = format!["https://api.kraken.com/0/public/Ticker?pair={}", pair];

        // Create request builder and send request
        match reqwest::blocking::get(url) {
            Ok(data) => {
                let json_str: Vec<u8> = data.bytes().unwrap().to_vec();
                let v: Value = serde_json::from_str(
                    &std::str::from_utf8(&json_str).unwrap()
                ).unwrap();

                // The key after "result" is the trading pair
                // as listed on Kraken. Rust JSON stuff is weird,
                // so we can't have "typed" JSON. This is how
                // we access the key.
                let symbol = v["result"]
                    .as_object()
                        .unwrap()
                            .iter()
                                .next()
                                    .unwrap()
                                        .0;

                // Lowest Ask
                // TODO Use currency 2 decimal places
                currency.buy = match &v["result"][symbol]["a"][0] {
                    Value::String(price) => price.parse::<f32>().unwrap(),
                    _ => panic!["Couldn't extract lowest ask!"]
                };

                // Highest Bid
                // TODO Use currency 2 decimal places
                currency.sell = match &v["result"][symbol]["b"][0] {
                    Value::String(price) => price.parse::<f32>().unwrap(),
                    _ => panic!["Couldn't extract lowest ask!"]
                };
            },
            Err(e) => {
                // TODO: What do we do if we can't get the data?
                eprintln!("{:?}", e);
            }
        }
    }
}

fn build_json_response(coinbase: &mut Exchange, kraken: &mut Exchange) -> Result<String, serde_json::Error> {
    let coinbase_json = coinbase.to_json();
    let kraken_json = kraken.to_json();

    let mut data: Vec<JsonExchange> = Vec::with_capacity(2);
    data.push(coinbase_json);
    data.push(kraken_json);

    let response = JsonResponse { data };

    serde_json::to_string(&response)
}

// The state for our app.
struct AppState {
    // exchange_data: Option<String>, // JSON String
    // exchange_data: Cell<Option<String>>, // JSON String
    exchange_data: RwLock<Option<String>>, // JSON String
}

#[get("/api/data")]
async fn serve_data(data: web::Data<AppState>) -> impl Responder {
    // TODO: Might need to acquire mutex lock first...
    let inner = data.exchange_data.read().unwrap();
    match &*inner {
        Some(json) => {
            let response = HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
                .body(json);
            return response;
        },
        None => {
            // TODO Return 404 or something?
            let response = HttpResponse::InternalServerError().body("No data found.");
            return response;
        }
    }
}

#[get("/styles.css")]
async fn style() -> impl Responder {
    let bytes = include_bytes!["../../frontend/styles.css"];
    let file = match String::from_utf8(bytes.to_vec()) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to fetch styles.css");
        }
    };
    let response = HttpResponse::Ok()
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
        .body(file);
    return response;
}

#[get("/images/{filename}")]
async fn image(web::Path(filename): web::Path<String>) -> impl Responder {
    let mut kraken_vec = None;
    let mut coinbase_vec = None;
    match &filename[..] {
        "kraken.jpg" => {
            kraken_vec = Some(include_bytes!["../../frontend/images/kraken.jpg"].to_vec());
        },
        "coinbase.svg" => {
            coinbase_vec = Some(include_bytes!["../../frontend/images/coinbase.svg"].to_vec());
        },
        _ => {
            return HttpResponse::InternalServerError().body(format!["Failed to fetch {}", filename]);
        }
    };

    if let Some(vec) = kraken_vec {
        return HttpResponse::Ok()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
            .body(vec);
    } else if let Some(vec) = coinbase_vec {
        return HttpResponse::Ok()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
            .header(http::header::CONTENT_TYPE, "image/svg+xml")
            .body(vec);
    }

    return HttpResponse::InternalServerError().body(format!["Failed to fetch {}", filename]);
}

#[get("/")]
async fn index() -> impl Responder {
    let bytes = include_bytes!["../../frontend/index.html"];
    let file = match String::from_utf8(bytes.to_vec()) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to fetch index.html");
        }
    };
    let response = HttpResponse::Ok()
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
        .body(file);
    return response;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_data = web::Data::new(AppState {
        exchange_data: RwLock::new(None)
    });

    // At this point, we want to create a new thread,
    // move a clone of json_data in (it uses Arc anyways)
    // and update is periodically

    let app_state_copy = json_data.clone();

    thread::spawn(move || {
        // Contained in here is a call to the exchanges
        let mut coinbase = Exchange::new("Coinbase");
        let mut kraken = Exchange::new("Kraken");

        loop {
            call_coinbase(&mut coinbase);
            call_kraken(&mut kraken);

            let json = match build_json_response(&mut coinbase, &mut kraken) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("{:?}", e);
                    panic!["Something went wrong creating JSON response."];
                }
            };
            let mut w = app_state_copy.exchange_data.write().unwrap();
            *w = Some(json);
            drop(w);
            thread::sleep(Duration::from_secs(2));
        }
    });

    HttpServer::new(move ||
        App::new()
            .app_data(json_data.clone())
            .service(index)
            .service(image)
            .service(style)
            .service(serve_data)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
