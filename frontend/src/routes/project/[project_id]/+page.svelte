<script lang="ts">
	import type { Report } from "$lib/models";
	import type { PageProps } from "./$types";
	let { data }: PageProps = $props();
	import type { ZodSafeParseResult } from "zod";

	const get_report_colors = (level: string): string => {
		switch (level) {
			case "TRACE":
				return "bg-pink-500 text-black";
				break;
			case "DEBUG":
				return "bg-blue-500 text-black";
				break;
			case "INFO":
				return "bg-green-500 text-black";
				break;
			case "WARN":
				return "bg-yellow-500 text-black";
				break;
			case "ERROR":
				return "bg-red-500 text-black";
				break;
		}
		return "bg-gray-500 text-black";
	};
</script>

{#await data.reports}
	<p>Loading...</p>
{:then result: ZodSafeParseResult<Report[]>}
	{#if !result.success}
		{@const _ = console.error(result.error)}
		<div class="text-xl text-destructive">
			What Something went wrong while trying to parse the backend response
		</div>
	{:else}
		<div class="mt-[20px] mb-[50px] flex flex-col">
			{#each result.data as report: Report}
				{@const received_at = new Date(report.received_at)}
				{@const reported_at = new Date(report.reported_at)}
				<div class="flex items-center justify-start gap-[12px]">
					<div
						class={`flex w-fit  items-center justify-start gap-[12px] border border-primary p-2 ${get_report_colors(report.level)}`}
					>
						<p>
							{reported_at.getDate()}/{reported_at.getMonth()}/{reported_at.getFullYear()}:{reported_at
								.getHours()
								.toString()
								.padStart(2, "0")}:{reported_at
								.getMinutes()
								.toString()
								.padStart(2, "0")}:{reported_at.getSeconds().toString().padStart(2, "0")}
						</p>
						<p>
							{received_at.getDate()}/{received_at.getMonth()}/{received_at.getFullYear()}:{received_at
								.getHours()
								.toString()
								.padStart(2, "0")}:{received_at
								.getMinutes()
								.toString()
								.padStart(2, "0")}:{received_at.getSeconds().toString().padStart(2, "0")}
						</p>
						<p class="">
							{report.level.toUpperCase()}
						</p>
					</div>
					<div>{report.body}</div>
				</div>
			{/each}
		</div>
	{/if}
{/await}
