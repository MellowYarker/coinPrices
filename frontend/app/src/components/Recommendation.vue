<template>
  <div class="recommendation">
    <div class="recommendation-title">{{ action }}</div>
    <span class="recommendation-section"
      v-for="rec in recommendations"
      :key="rec.symbol"
    >
      <div>{{ rec.symbol }} {{ rec.exchange }}</div>
    </span>
  </div>
</template>

<script>
	export default {
		name: 'Recommendation',
		props: {
      action: String,
			recommendations: Array(Object)
		}
	}
</script>

<style scoped>
  .recommendation {
    display: inline-block;
    padding-top: 5px;
    position: relative;
    text-align: center;
    width: 100%;
  }
  .recommendation-section {
    display: block;
    padding-top: 5px;
    position: relative;
    text-align: center;
  }
  .recommendation-title {
    font-size: 25px;
    font-weight: bold;
    padding-bottom: 10px;
    text-align: center;
  }
</style>