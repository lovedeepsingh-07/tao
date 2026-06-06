<script lang="ts">
	import type { PageProps } from "./$types";
	import type { Project } from "$lib/models";
	import type { ZodSafeParseResult } from "zod";
	import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
	import { env } from "$env/dynamic/public";
	import { delete_project, create_project } from "$lib";
	import { invalidateAll } from "$app/navigation";

	const handle_delete_button_click = async (project_id: string) => {
		const result = await delete_project(
			fetch,
			PUBLIC_APP_RUN_METHOD,
			project_id,
			env.PUBLIC_API_URL
		);
		await invalidateAll();
	};

	const handle_create_project_click = async () => {
		const raw_name = prompt("Enter project name: ");

		const project_name = raw_name ? raw_name.trim() : "";

		if (!project_name) {
			alert("You have to enter valid input project name");
			return;
		}
		const res = await create_project(
			fetch,
			PUBLIC_APP_RUN_METHOD,
			project_name,
			env.PUBLIC_API_URL
		);
		if (!res.success) {
			alert("Something went wrong!");
			console.error(res.error);
		} else {
			console.log(res.data);
		}
	};

	let { data }: PageProps = $props();
</script>

<div class="mt-[48px]">
	<h1 class="text-2xl">Your Projects</h1>
	{#await data.project_list}
		<p>loading...</p>
	{:then result: ZodSafeParseResult<Project[]>}
		{#if result.success !== true}
			<p>unable to parse data sent from the server</p>
		{:else}
			<div class="flex flex-col items-start p-4">
				{#each result.data as project: Project}
					<div class="flex items-center justify-start gap-[10px]">
						<button
							class="ixed right-[20px] bottom-[20px] rounded rounded-lg border bg-destructive px-2 py-1 text-destructive-foreground hover:bg-destructive/80"
							onclick={async () => {
								await handle_delete_button_click(project.id);
							}}>X</button
						>
						<a
							class="rounded rounded-lg border border-primary px-4 py-2"
							href={`/project/${project.id}`}
						>
							{project.name}
						</a>
					</div>
				{/each}
			</div>
		{/if}
	{/await}
</div>
<button
	class="fixed right-[20px] bottom-[20px] rounded rounded-lg border bg-primary px-2 py-1 text-primary-foreground hover:bg-primary/80"
	onclick={handle_create_project_click}
>
	New Project</button
>
