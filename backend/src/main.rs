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

/* Things we want to do:
 *  Call the APIs, fill in our structs.
 * */
use actix_web::client::Client;
use serde::{self, Deserialize};
use serde_json::{self, Value};

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
}

#[derive (Deserialize, Debug)]
struct coinbaseData {
    base: String,
    currency: String,
    amount: String, // TODO convert this to currency 2 decimal places!
}

#[derive (Deserialize, Debug)]
struct coinbaseResponse {
    data: coinbaseData,
}


async fn call_coinbase(coinbase: &mut Exchange) {
    let fiat = "USD";
    let mut client = Client::default();

    let actions = ["buy", "sell"];

    for currency in &mut coinbase.currencies {
        for action in &actions {
            let pair = format!["{}-{}", currency.symbol, fiat];
            let url = format!["https://api.coinbase.com/v2/prices/{}/{}", pair, action];

            // Create request builder and send request
            let response = client.get(url)
                .header("User-Agent", "actix-web/3.0")
                .send()
                .await;

            match response {
                Ok(mut data) => {
                    let json: Result<coinbaseResponse, _> = data.json().await;
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

async fn call_kraken(kraken: &mut Exchange) {
    let fiat = "USD";
    let mut client = Client::default();

    for currency in &mut kraken.currencies {
        // Generate the url for each currency.
        // Ideally, we would produce these once and reuse them,
        // but it would be tricky to access them in a loop.
        let pair = format!["{}{}", currency.symbol, fiat];
        let url = format!["https://api.kraken.com/0/public/Ticker?pair={}", pair];
        // Create request builder and send request
        let response = client.get(url)
            .header("User-Agent", "actix-web/3.0")
            .send()
            .await;


        match response {
            Ok(mut data) => {
                let json_str: Vec<u8> = data.body().await.unwrap().to_ascii_lowercase();
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

#[actix_web::main]
async fn main() {
    let mut coinbase = Exchange::new("Coinbase");
    let mut kraken = Exchange::new("Kraken");

    println!("{:?}", coinbase);
    println!("{:?}", kraken);
    // We know the URLs don't change,
    // we might as well set them once and resuse.
    call_coinbase(&mut coinbase).await;
    call_kraken(&mut kraken).await;
    println!("{:?}", coinbase);
    println!("{:?}", kraken);
}
