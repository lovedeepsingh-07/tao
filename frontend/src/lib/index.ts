export const fetch_health = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	API_URL?: string
): string => {
	const fetch_url = app_run_method == "api" ? `${API_URL}/health` : "/health";
	const res = await svelte_fetch(fetch_url);
	return await res.text();
};

export const get_reports = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	API_URL?: string
): string => {
	const fetch_url = app_run_method == "api" ? `${API_URL}/api/report` : "/api/report";
	const res = await svelte_fetch(fetch_url, { method: "GET" });
	return await res.json();
};
