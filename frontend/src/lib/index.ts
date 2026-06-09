import type { Report, Project } from "$lib/models";
import { ReportModel, ProjectModel } from "$lib/models";
import type { ZodSafeParseResult } from "zod";
import { z } from "zod";

export const fetch_health = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	API_URL?: string
): Promise<string> => {
	const fetch_url: string = app_run_method == "api" ? `${API_URL}/api/health` : "/api/health";
	const res: Response = await svelte_fetch(fetch_url);
	const data: string = await res.text();
	return data;
};

export const get_reports = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	project_id: string,
	API_URL?: string
): Promise<ZodSafeParseResult<Report[]>> => {
	const fetch_url: string =
		app_run_method == "api"
			? `${API_URL}/api/project/${project_id}/report`
			: `/api/project/${project_id}/report`;
	const res: Response = await svelte_fetch(fetch_url, { method: "GET" });
	const data: unknown = await res.json();
	return z.array(ReportModel).safeParse(data);
};

export const get_projects = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	API_URL?: string
): Promise<ZodSafeParseResult<Project[]>> => {
	const fetch_url: string = app_run_method == "api" ? `${API_URL}/api/project` : `/api/project`;
	const res: Response = await svelte_fetch(fetch_url, { method: "GET" });
	const data: unknown = await res.json();
	return z.array(ProjectModel).safeParse(data);
};

export const create_project = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	project_name: string,
	API_URL?: string
): Promise<ZodSafeParseResult<Project>> => {
	const fetch_url: string = app_run_method == "api" ? `${API_URL}/api/project` : `/api/project`;
	const res: Response = await svelte_fetch(fetch_url, {
		method: "POST",
		body: JSON.stringify({
			name: project_name
		}),
		headers: { "Content-Type": "application/json" }
	});
	const data: unknown = await res.json();
	return ProjectModel.safeParse(data);
};

export const delete_project = async (
	svelte_fetch: typeof fetch,
	app_run_method: string,
	project_id: string,
	API_URL?: string
): Promise<ZodSafeParseResult<Project>> => {
	const fetch_url: string =
		app_run_method == "api" ? `${API_URL}/api/project/${project_id}` : `/api/project/${project_id}`;
	const res: Response = await svelte_fetch(fetch_url, {
		method: "DELETE"
	});
	const data: unknown = await res.json();
	return ProjectModel.safeParse(data);
};
