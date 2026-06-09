<script lang="ts">
	import "./layout.css";
	import { ModeWatcher } from "mode-watcher";
	import type { PageProps } from "./$types";
	import { goto } from "$app/navigation";

	let { data, children }: PageProps = $props();
</script>

<svelte:head>
	<title>tao</title>
	<!-- <link rel="icon" type="image/png" href="/icon.png" /> -->
</svelte:head>

<ModeWatcher defaultMode={"dark"} />
<div class="mx-auto w-full max-w-[78%]">
	{#await data.is_healthy}
		<p class="text-2xl">Loading....</p>
	{:then is_healthy}
		{#if is_healthy == true}
			{@render children()}
		{:else}
			{@const _ = goto("/")}
			<p class="text-2xl">Sorry, backend is down</p>
		{/if}
	{/await}
</div>
