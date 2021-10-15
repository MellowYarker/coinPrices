import json, requests, signal, sys, time

"""
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
"""

def call_kraken(exchange: dict):
    """
        Sends a GET request to the kraken Ticker endpoint.
        We modify the dictionary that we pass as an argument, on error,
        we return an IOError.
    """
    base_url = "https://api.kraken.com/0/public/Ticker?pair={}"

    currencies = ["BTC", "ETH"]
    fiat = "USD"

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

        # Modifies or creates dictionary values for the given currency
        update_exchange(exchange, currency, lowest_ask, highest_bid)

    return None

def call_coinbase(exchange: dict):
    """
        Sends a GET request to the coinbase prices endpoint.
        We modify the dictionary that we pass as an argument.
        TODO: Error checking on bad response.
    """
    base_url = "https://api.coinbase.com/v2/prices/{}/{}"

    currencies = ["BTC", "ETH"]
    fiat = "USD"
    actions = ["buy", "sell"]

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

        # Modifies or creates dictionary values for the given currency
        update_exchange(exchange, currency, lowest_ask, highest_bid)

    return None

# Modifies or creates dictionary values for the given currency
# If the currency is found, we update the data,
# otherwise, we create a new dictionary of prices.
def update_exchange(exchange: dict, currency: str, lowest_ask, highest_bid):
    if currency in exchange:
        # Modify existing values
        exchange[currency]["buy"] = lowest_ask
        exchange[currency]["sell"] = highest_bid
    else:
        # Create new dict
        prices = dict()
        prices["buy"] = lowest_ask
        prices["sell"] = highest_bid
        exchange[currency] = prices

# Quiet termination
def signal_handler(sig, frame):
    sys.exit(0)

def runCrawler():
    # Signal handling
    signal.signal(signal.SIGINT, signal_handler)

    with open ("prices.json", "w") as f:
        # Initializing our dictionary.
        # This will eventually become JSON.
        exchanges = dict()
        exchanges["kraken"] = dict()
        exchanges["coinbase"] = dict()

        while(True):
            err = call_kraken(exchanges["kraken"])
            if type(err) is IOError:
                # TODO: Handle errors properly
                raise(err)
                exit(1)

            err = call_coinbase(exchanges["coinbase"])
            if err is not None:
                exit(1)

            final_string = json.dumps(exchanges) # convert dict to JSON

            # Overwrite our data
            f.write(final_string)
            f.seek(0)
            time.sleep(2)
