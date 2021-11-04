<template>
	<div class="hide-stale-data">
		<button class="connect-button" @click="onClick">Attempt server connection</button>
		<div style="padding-top: 10px;">
			<span class="server-error">There seems to be an issue with the server. Try reconnecting.</span>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Disconnected',
		props: {
			name: function() {
				return 'connect'
			},
			type: Function,
			required: function() {
				true
			}
		},
		methods: {
			onClick: function() {
				this.$emit('requestConnection')
			}
		},
	}
</script>

<style scoped>
  .connect-button {
    height: 40px;
    position: relative;
    top: 5%;
  }

  .hide-stale-data {
    display: grid;
    justify-content: center;
  }

  .server-error {
    align-items: center;
    display: flex;
    justify-content: center;
    text-align: center;
  }

</style>