<script lang="ts">
	import type { PageProps } from "./$types";
	let { data }: PageProps = $props();

	const get_report_colors = (kind: string): string => {
		switch (kind) {
			case "debug":
				return "bg-blue-500 text-black";
				break;
			case "info":
				return "bg-green-500 text-black";
				break;
			case "warn":
				return "bg-yellow-500 text-black";
				break;
			case "error":
				return "bg-red-500 text-black";
				break;
		}
	};
</script>

{#await data.reports}
	<p>Loading...</p>
{:then reports}
	<div class="mt-[20px] mb-[50px] flex flex-col">
		{#each reports as report}
			{@const created_at = new Date(report.created_at)}
			<div class="flex items-center justify-start gap-[12px]">
				<div
					class={`flex w-full max-w-[248px] items-center justify-start gap-[12px] border border-primary p-2 ${get_report_colors(report.kind)}`}
				>
					<p>
						{created_at.getDate()}/{created_at.getMonth()}/{created_at.getFullYear()}:{created_at
							.getHours()
							.toString()
							.padStart(2, "0")}:{created_at.getMinutes().toString().padStart(2, "0")}:{created_at
							.getSeconds()
							.toString()
							.padStart(2, "0")}
					</p>
					<p class="">
						{report.kind.toUpperCase()}
					</p>
				</div>
				<div>{report.body}</div>
			</div>
		{/each}
	</div>
{/await}
