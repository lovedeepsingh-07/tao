export const ssr = false;

import { PUBLIC_APP_RUN_METHOD } from "$env/static/public";
import { env } from "$env/dynamic/public";
import { fetch_health } from "$lib";
import type { LayoutLoad, LayoutLoadEvent } from "./$types";

const is_healthy = async (load_event: LayoutLoadEvent): Promise<boolean> => {
	let is_healthy: boolean = false;
	try {
		const health_state = await fetch_health(
			load_event.fetch,
			PUBLIC_APP_RUN_METHOD,
			env.PUBLIC_API_URL
		);
		// TODO: remove this 'type shi'
		is_healthy = health_state === "type shi";
	} catch (error) {
		console.error(error);
	}

	return is_healthy;
};

export const load: LayoutLoad = async (load_event) => {
	return {
		is_healthy: is_healthy(load_event)
	};
};
