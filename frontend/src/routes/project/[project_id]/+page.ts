import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
import { env } from "$env/dynamic/public";
import { get_reports } from "$lib";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (load_event) => {
	const { project_id } = load_event.params;
	return {
		project_id,
		reports: get_reports(load_event.fetch, PUBLIC_APP_RUN_METHOD, project_id, env.PUBLIC_API_URL)
	};
};
