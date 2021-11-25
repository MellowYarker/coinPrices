use actix_web::{get, http, HttpServer, HttpResponse, web, App, Responder};
use reqwest::blocking;
use serde::{self, Serialize, Deserialize};
use serde_json::{self, Value};
use std::{env, time::Duration, sync::RwLock, thread};

/* The following (Json-prefixed) structs are used for (de)serializing JSON */

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
struct Currency {
    symbol: String,
    buy: f32, // TODO use money lib
    sell: f32, // TODO use money lib
}

impl Currency {
    fn new(symbol: &str) -> Self {
        Currency {
            symbol: symbol.to_string(), // (BTC / ETH)
            buy: 0.0,
            sell: 0.0,
        }
    }
}

#[derive (Debug)]
struct Exchange {
    name: String, // exchange name
    currencies: Vec<Currency>
}

impl Exchange {
    fn new(name: &str) -> Self {
        let currencies: Vec<Currency> = vec![Currency::new("BTC"), Currency::new("ETH")];

        Exchange {
            name: name.to_string(),
            currencies
        }
    }

    // Converts this exchange struct to JSON
    /*
     *  {
     *     name: exchange's name,
     *     BTC: {
     *          buy: x_1,
     *          sell: x_1,
     *     },
     *     ETH: {
     *          buy: y_1,
     *          sell: y_2,
     *     },
     *  }
     **/
    fn to_json(&self) -> JsonExchange {
        let BTC = JsonCurrency { buy: self.currencies[0].buy, sell: self.currencies[0].sell };
        let ETH = JsonCurrency { buy: self.currencies[1].buy, sell: self.currencies[1].sell };

        JsonExchange {name: self.name.clone(), BTC, ETH}
    }
}

/* These Coinbase prefixed structs are for
 * deserializing JSON from coinbase's API.
 **/
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


/* Send a blocking GET request to Coinbase's API.
 * We can't use async here because it doesn't
 * implement the SEND trait, and we're using
 * multiple threads.
 **/
fn call_coinbase(coinbase: &mut Exchange) {
    let fiat = "USD";

    let actions = ["buy", "sell"];

    for currency in &mut coinbase.currencies {
        for action in &actions {
            let pair = format!["{}-{}", currency.symbol, fiat];
            let url = format!["https://api.coinbase.com/v2/prices/{}/{}", pair, action];

            // Send a BLOCKING request.
            match blocking::get(url) {
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

/* Send a blocking GET request to Kraken's API.
 * We can't use async here because it doesn't
 * implement the SEND trait, and we're using
 * multiple threads.
 **/
fn call_kraken(kraken: &mut Exchange) {
    let fiat = "USD";

    for currency in &mut kraken.currencies {
        // Generate the url for each currency.
        // Ideally, we would produce these once and reuse them,
        // but it would be tricky to access them in a loop.
        let pair = format!["{}{}", currency.symbol, fiat];
        let url = format!["https://api.kraken.com/0/public/Ticker?pair={}", pair];

        // Send blocking request.
        match blocking::get(url) {
            Ok(data) => {
                let json_str: Vec<u8> = data.bytes().unwrap().to_vec();
                let v: Value = serde_json::from_str(
                    &std::str::from_utf8(&json_str).unwrap()
                ).unwrap();

                /* The key after "result:" is the trading pair
                 * as listed on Kraken (Ex. XXBTZUSD).
                 *
                 * Becaue Serde JSON's typed json wants to know
                 * the key name in advance, we would need different
                 * structs for each currency pair. Instead, we just
                 * use UNtyped json (treat as String), and extract the symbol here.
                **/
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
                    _ => panic!["Couldn't extract highest bid!"]
                };
            },
            Err(e) => {
                // TODO: What do we do if we can't get the data?
                eprintln!("{:?}", e);
            }
        }
    }
}

/* Here, we JSONify our exchange structs, and merge them to create
 * the final JSON response that we will serve to clients.
 **/
fn build_json_response(coinbase: &mut Exchange, kraken: &mut Exchange) -> Result<String, serde_json::Error> {
    let coinbase_json = coinbase.to_json();
    let kraken_json = kraken.to_json();

    let data: Vec<JsonExchange> = vec![coinbase_json, kraken_json];

    let response = JsonResponse { data };

    serde_json::to_string(&response)
}

/* Everything below this line is Actix-web stuff. */

// The state for our app.
struct AppState {
    exchange_data: RwLock<Option<String>>, // JSON String
}

/* API endpoint that client's make requests to. */
#[get("/api/data")]
async fn serve_data(data: web::Data<AppState>) -> impl Responder {
    // Readers have shared access, will stall when writer has lock.
    let inner = data.exchange_data.read().unwrap();
    match &*inner {
        Some(json) => {
            HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET")
                .body(json)
        },
        None => {
            // TODO Return 404 or something?
            HttpResponse::InternalServerError().body("No data found.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_data = web::Data::new(AppState {
        exchange_data: RwLock::new(None) // No JSON by default
    });

    // This is stored in an Arc, it's thread safe
    // and cloning == incrementing references.
    let app_state_copy = json_data.clone();

    // Thread that tries to update data from exchanges once every 3 sec.
    thread::spawn(move || {
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
            // Acquire the write lock, will wait until all readers release.
            // No readers can acquire lock after writer implies it wants
            // the lock.
            let mut w = app_state_copy.exchange_data.write().unwrap();
            *w = Some(json);
            drop(w); // Explicit drop so that we don't wait 3 sec duration to drop

            thread::sleep(Duration::from_secs(3));
        }
    });

    let HOST = env::var("HOST").expect("Host not set");
    let PORT = env::var("PORT").expect("Port not set");

    HttpServer::new(move ||
        App::new()
            .app_data(json_data.clone())
            .service(serve_data) // api endpoint, this stays
    )
    // .bind("127.0.0.1:8989")?
    .bind(format!("{}:{}", HOST, PORT))?
    .run()
    .await
}
