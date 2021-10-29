<template>
	<span class="price-section">
		<div class="currency"
			v-for="currency in exchange.currencies"
			:key="currency.id"
		>
				<div class="trade">
					<a v-bind:href=exchange.link class="exchange-link-buy">buy</a> 
					{{ currency.symbol }}: {{currency.buy}} USD
				</div>

				<div class="trade">
					<a v-bind:href=exchange.link class="exchange-link-sell">sell</a> 
					{{ currency.symbol }}: {{currency.sell}} USD
				</div>
		</div>
	</span>
</template>

<script>
	export default {
		name: 'PriceDisplay',
		props: {
			exchange: {
				id: Number,
				name: String,
				currencies: Array,
				image: String,
				link: String
			},
		}
	}
</script>

<style scoped>
	.currency {
		padding-left: 10px;
		padding-bottom: 15px;
	}

	.exchange-link-buy {
		background: lightgrey;
		border-color: lightgrey;
		border-radius: 5px;
		border-style: solid;
		color: cornflowerblue;
		padding-left: 4px;
		padding-right: 4px;
		text-decoration: none;
	}

	.exchange-link-sell {
		background: lightgrey;
		border-color: lightgrey;
		border-radius: 5px;
		border-style: solid;
		color: orangered;
		padding-left: 4px;
		padding-right: 4px;
		text-decoration: none;
	}

	.price-section {
		display: inline-block;
		padding: 30px;
	}

	.trade {
		padding-bottom: 10px;
	}
</style>