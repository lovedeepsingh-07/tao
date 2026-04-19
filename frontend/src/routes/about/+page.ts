const fetch_health = async (server_utils) => {
	const res = await server_utils.fetch("/health");
	return await res.text();
};

export const load = async (server_utils) => {
	return {
		health: fetch_health(server_utils)
	};
};
