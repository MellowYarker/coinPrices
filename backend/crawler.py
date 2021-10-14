import requests
import json
import time

"""
Kraken's API:
    https://api.kraken.com/0/public/Ticker?pair=BTCUSD
    https://api.kraken.com/0/public/Ticker?pair=ETHUSD

We can get the data like so:
    json = (url blah blah).json()
    lowest_ask = json[a][0]
    highest_bid = json[b][0]

Coinbase's API:
    https://api.coinbase.com/v2/prices/BTC-USD/buy
    https://api.coinbase.com/v2/prices/BTC-USD/sell

    https://api.coinbase.com/v2/prices/ETH-USD/buy
    https://api.coinbase.com/v2/prices/ETH-USD/sell

"""

def call_kraken():
    """
        This function sends a GET request to the kraken Ticker endpoint.
        The response type is a list of (symbol, ask, sell) tuples, or
        an IOError.
    """
    base_url = "https://api.kraken.com/0/public/Ticker?pair={}"

    currencies = ["BTC", "ETH"]
    fiat = "USD"
    results = list()

    for currency in currencies:
        pair = "{}{}".format(currency, fiat)
        url = base_url.format(pair)

        response = requests.get(url)
        json = response.json()

        # Got an error.
        if len(json["error"]) > 0:
            return IOError(json["errror"])

        # Extract the pair name
        key = next(iter(json["result"].keys()))

        json = json["result"][key]

        lowest_ask = json['a'][0]
        highest_bid = json['b'][0]

        results.append((currency, lowest_ask, highest_bid))

    return results

def call_coinbase():
    """
        This function sends a GET request to the coinbase Ticker endpoint.
        The response type is a list of (symbol, ask, sell) tuples, or
        an IOError.
    """
    base_url = "https://api.coinbase.com/v2/prices/{}/{}"

    currencies = ["BTC", "ETH"]
    fiat = "USD"
    actions = ["buy", "sell"]
    results = list()

    for currency in currencies:
        lowest_ask = None
        highest_bid = None
        for action in actions:
            pair = "{}-{}".format(currency, fiat)
            url = base_url.format(pair, action)

            response = requests.get(url)
            json = response.json()['data']
            price = json['amount']

            # Coinbase does it the other way
            # It gives you the market price for buy/sell
            if action == "buy":
                lowest_ask = price
            elif action == "sell":
                highest_bid = price

        results.append((currency, lowest_ask, highest_bid))

    return results

if __name__ == "__main__":

    with open ("prices.json", "w") as f:
        while(True):
            kraken_prices = call_kraken()
            # print("Kraken:");
            # for x in kraken_prices:
            #     print(f"\t{x}")

            kraken_dict = dict()
            for x in kraken_prices:
                price_dict = dict()
                price_dict["sell"] = x[2]
                price_dict["buy"] = x[1]

                kraken_dict[x[0]] = price_dict

            coinbase_prices = call_coinbase()
            # print("Coinbase:");
            # for x in coinbase_prices:
            #     print(f"\t{x}")

            coinbase_dict = dict()
            for x in coinbase_prices:
                price_dict = dict()
                price_dict["sell"] = x[2]
                price_dict["buy"] = x[1]

                coinbase_dict[x[0]] = price_dict

            result = dict()

            # data: list of exchanges
            result["kraken"] = kraken_dict
            result["coinbase"] = coinbase_dict

            final_string = json.dumps(result)

            # Overwrite our data
            f.write(final_string)
            f.seek(0)
            time.sleep(2)


    """
    The resulting JSON should be of the following form:
    "kraken": {
        "btc": {
            "buy": x,
            "sell": x
        },
        "eth": {
            "buy": x,
            "sell": x
        }
    },
    "coinbase": {
        "btc": {
            "buy": x,
            "sell": x
        },
        "eth": {
            "buy": x,
            "sell": x
        }
    }
    """
