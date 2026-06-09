import { z } from "zod";

export const ProjectModel = z.object({
	id: z.uuidv4(),
	name: z.string().default("")
});
export type Project = z.infer<typeof ProjectModel>;

export const ReportLevelModel = z.enum(["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]);
export type ReportLevel = z.infer<typeof ReportLevelModel>;

export const ReportModel = z.object({
	id: z.uuidv4(),
	project_id: z.uuidv4(),
	reported_at: z.string().default(""),
	received_at: z.string().default(""),
	body: z.string().default(""),
	location: z.string(),
	level: ReportLevelModel,
	used_memory: z.number(),
	total_memory: z.number(),
	cpu_percent: z.number()
});
export type Report = z.infer<typeof ReportModel>;
