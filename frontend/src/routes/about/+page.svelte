<script lang="ts">
	let { data } = $props();
</script>

<h1 class="text-2xl">About Page</h1>
{#await data.health}
	<p>Loading...</p>
{:then health}
	<p>{health}</p>
{/await}
