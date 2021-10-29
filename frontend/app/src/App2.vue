<template>
  <div>
    <div>
        <span style="display: flex; justify-content: center; align-items: center; text-align: center;">{{ status_message }}</span>
    </div>
    <!-- TODO: DeadNetwork Component -->
    <div v-if="deadNetwork" class="hide-stale-data">
      <button class="connect-button" @click="attemptConnection">Attempt server connection</button>
      <div style="padding-top: 10px;">
        <span class="server-error">There seems to be an issue with the server. Try reconnecting.</span>
      </div>
    </div>
    <!-- TODO: Do we want a Homepage component? -->
    <div v-else class="homepage">
      <ExchangeBanner
        v-for="exchange in exchanges"
        :exchange="exchange"
        :key="exchange.id"/>
      <PriceDisplay
        v-for="exchange in exchanges"
        :exchange="exchange"
        :key="exchange.id"/>
      <Recommendations
        v-if="reqsMade"
        :recs="recs"/>
    </div>
  </div>
</template>

<script>
import ExchangeBanner from './components/ExchangeBanner.vue'
import PriceDisplay from './components/PriceSection.vue'
import Recommendations from './components/Recommendations.vue'

export default {
  name: 'App2',
  components: {
    ExchangeBanner,
    PriceDisplay,
    Recommendations
  },
  data() {
    return {
      status_message: null,
      // TODO: I don't like wasting so much space
      // with these default objects. They're so similar by default,
      // so I feel like we can generate them via computed: {}
      exchanges: [
        {
          id: 0,
          name: "Kraken",
          currencies: [
            {
              symbol: "BTC",
              buy: null,
              sell: null
            },
            {
              symbol: "ETH",
              buy: null,
              sell: null
            }
          ],
          image: "../assets/kraken.jpg",
          link: "https://www.kraken.com"
        },
        {
          id: 1,
          name: "Coinbase",
          currencies: [
            {
              symbol: "BTC",
              buy: null,
              sell: null
            },
            {
              symbol: "ETH",
              buy: null,
              sell: null
            }
          ],
          image: "../assets/coinbase.svg",
          link: "https://www.coinbase.com"
        }
      ],
      // TODO: these are so similar I think we should just generate them later in a loop
      recs: [
        { 
          action: "Best Buy",
          recommendations: [
            {
              symbol: "BTC",
              exchange: null
            },
            { symbol: "ETH",
              exchange: null
            },
          ]
        },
        { 
          action: "Best Sell",
          recommendations: [
            {
              symbol: "BTC",
              exchange: null
            },
            { symbol: "ETH",
              exchange: null
            },
          ]
        },
      ],
      last_update: null,
      deadNetwork: true,  // the connection to the server is down.
      reqsMade: false     // true if we've made a recommendation
    } 
  },
  beforeMount() {
    // When we first load, connect to the server.
    this.attemptConnection()

    // Try to request the data every 2 seconds.
    // If the connection is down, we increment the number of seconds since
    // the last update so the user know's their data is stale.
    window.setInterval(() => {
      let now = new Date()
      if (this.last_update == null) {
        this.last_update = now
      }

      // We are connected
      if (!this.deadNetwork) {
        this.callBackend();
        this.last_update = now
      }

      let prev = this.last_update
      this.status_message = "Last update: " + Math.round(((now - prev) / 1000)) + " seconds ago.";
    }, 2000)
  },
  methods: {
    findRecommendations: function () {
      let kraken = this.exchanges[0]
      let coinbase = this.exchanges[1]

      // Set the recommendations.
      for (let i = 0; i < 2; i++) {
        // Buy recommendation, we buy wherever is cheaper
        if (kraken.currencies[i].buy < coinbase.currencies[i].buy) {
          this.recs[0].recommendations[i] = {
            symbol: kraken.currencies[i].symbol,
            exchange: kraken.name
          }
        } else {
          this.recs[0].recommendations[i] = {
            symbol: coinbase.currencies[i].symbol,
            exchange: coinbase.name
          }
        }
        // Sell recommendation, we sell wherever is most expensive
        if (kraken.currencies[i].sell < coinbase.currencies[i].sell) {
          this.recs[1].recommendations[i] = {
            symbol: coinbase.currencies[i].symbol,
            exchange: coinbase.name
          }
        } else {
          this.recs[1].recommendations[i] = {
            symbol: kraken.currencies[i].symbol,
            exchange: kraken.name
          }
        }
      }
      this.reqsMade = true;
    },

    attemptConnection: function () {
      this.status_message = "Updating"
      this.callBackend()
    },

    callBackend: function () {
      fetch("http://localhost:8989/api/data")
      .then(response => {
        return response.json()
      })
      .then(json => {
        this.deadNetwork = false;
        let data = json["data"];
        for (let x in data) {
          let exchange = data[x];
          if (exchange["name"] == "Kraken") {
            this.exchanges[0].currencies[0].buy = exchange["BTC"]["buy"];
            this.exchanges[0].currencies[1].buy = exchange["ETH"]["buy"];
            this.exchanges[0].currencies[0].sell = exchange["BTC"]["sell"];
            this.exchanges[0].currencies[1].sell = exchange["ETH"]["sell"];
          } else if (exchange["name"] == "Coinbase") {
            this.exchanges[1].currencies[0].buy = exchange["BTC"]["buy"];
            this.exchanges[1].currencies[1].buy = exchange["ETH"]["buy"];
            this.exchanges[1].currencies[0].sell = exchange["BTC"]["sell"];
            this.exchanges[1].currencies[1].sell = exchange["ETH"]["sell"];
          }
        }

        this.findRecommendations();

        return;
      })
      .catch(response => {
        this.deadNetwork = true;
        console.log(response);
      })
    }
  }
}
</script>

<style>
  .connect-button {
    height: 40px;
    position: relative;
    top: 5%;
  }

  .hide-stale-data {
    display: grid;
    justify-content: center;
  }

  .homepage {
    padding-left: 35%;
    padding-top: 5%;
    position: absolute;
    width: 500px;
  }

  .server-error {
    align-items: center;
    display: flex;
    justify-content: center;
    text-align: center;
  }
</style>
