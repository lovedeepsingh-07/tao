<script lang="ts">
	import "./layout.css";
	import { ModeWatcher } from "mode-watcher";

	let { children } = $props();
</script>

<svelte:head>
	<title>tao</title>
	<!-- <link rel="icon" type="image/png" href="/icon.png" /> -->
</svelte:head>

<ModeWatcher defaultMode={"dark"} />
<div class="mx-auto w-full max-w-[78%]">
	{@render children()}
</div>
