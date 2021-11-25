<template>
	<span class="exchange-logo-span">
		<a :href=exchange.link>
			<img class="exchange-logo" :src=exchange.image>
		</a>
	</span>
</template>

<script>
	export default {
		name: 'ExchangeBanner',
		props: {
			exchange: {
				id: Number,
				name: String,
				currencies: Array,
				image: String,
				link: String
			},
		},
	}
</script>

<style scoped>
	.exchange-logo {
		height: 60px;
		width: 100px;
	}
	.exchange-logo-span {
		display: inline-block;
		position: relative;
		text-align: -webkit-center;
		width: 50%;
	}
</style>