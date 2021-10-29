<template>
  <div class="recommendations">
    <div class="buy-or-sell">
      <Recommendation
        v-for="rec in recs"
        :action="rec.action"
        :recommendations="rec.recommendations"
        :key="rec.action"
      ></Recommendation>
    </div>
  </div>
</template>

<script>
import Recommendation from './Recommendation.vue'

export default {
  name: 'Recommendations',
  components: {
    Recommendation
  },
  props: {
    recs: Array(Object),
  }
}
</script>

<style scoped>
  .buy-or-sell {
    display: flex;
    position: relative;
    width: 100%;
  }

  .recommendations {
    border: 3px;
    border-color: black;
    border-style: solid;
    display: flex;
    padding: 10px;
    width: 474px; /* homepage width - recommendations.padding * 2 - recommendations.border * 2*/
  }
</style>