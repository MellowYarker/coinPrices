import requests
import json
import time
import signal
import sys

"""
Kraken's API:
    https://api.kraken.com/0/public/Ticker?pair=BTCUSD

Coinbase's API:
    https://api.coinbase.com/v2/prices/BTC-USD/buy
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

# Quiet termination
def signal_handler(sig, frame):
    sys.exit(0)

# TODO: This works, but is generally pretty messy and we can do better.
# Some simple things we can fix:
#   1. call_x() functions can return class objects or dictionaries
#   2. We don't have to lose our dictionaries each time we loop,
#      create them once and reuse/modify them!
#   3. We do the same process for each exchange, just make a function.
def runCrawler():
    # Signal handling
    signal.signal(signal.SIGINT, signal_handler)

    with open ("prices.json", "w") as f:
        while(True):
            kraken_prices = call_kraken()
            if type(kraken_prices) is IOError:
                # TODO: We got an error
                raise(kraken_prices)
                continue

            kraken_dict = dict()
            for x in kraken_prices:
                price_dict = dict()
                price_dict["sell"] = x[2]
                price_dict["buy"] = x[1]

                kraken_dict[x[0]] = price_dict

            coinbase_prices = call_coinbase()
            if type(coinbase_prices) is IOError:
                # TODO: We got an error
                raise(coinbase_prices)
                continue

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
            """
