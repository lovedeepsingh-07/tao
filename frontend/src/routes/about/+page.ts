import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
import { PUBLIC_API_URL } from "$env/static/public";
import { fetch_health } from "$lib";

export const load = async (server_utils) => {
	return {
		health: fetch_health(server_utils, PUBLIC_APP_RUN_METHOD, PUBLIC_API_URL)
	};
};
