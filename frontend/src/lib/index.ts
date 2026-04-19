export const fetch_health = async (server_utils, app_run_method: string, API_URL: string) => {
	const fetch_url = app_run_method == "api" ? `${API_URL}/health` : "/health";
	const res = await server_utils.fetch(fetch_url);
	return await res.text();
};
