import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
import { env } from "$env/dynamic/public";
import { get_reports } from "$lib";

export const load = async (server_utils) => {
	return {
		reports: get_reports(server_utils.fetch, PUBLIC_APP_RUN_METHOD, env.PUBLIC_API_URL)
	};
};
