import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
import { env } from "$env/dynamic/public";
import { fetch_health } from "$lib";

export const load = async (server_utils) => {
	return {
		health: fetch_health(server_utils, PUBLIC_APP_RUN_METHOD, env.PUBLIC_API_URL)
	};
};
