import { z } from "zod";

export const ProjectModel = z.object({
	id: z.uuidv4(),
	slug: z.string().default(""),
	name: z.string().default("")
});
export type Project = z.infer<typeof ProjectModel>;

export const ReportKindModel = z.enum(["DEBUG", "INFO", "WARN", "ERROR"]);
export type ReportKind = z.infer<typeof ReportKindModel>;

export const ReportModel = z.object({
	id: z.uuidv4(),
	project_id: z.uuidv4(),
	reported_at: z.string().default(""),
	received_at: z.string().default(""),
	release: z.string(),
	body: z.string().default(""),
	stack_trace: z.string().optional(),
	kind: ReportKindModel,
	memory_usage: z.number(),
	cpu_percent: z.number(),
	disk_usage_percent: z.number()
});
export type Report = z.infer<typeof ReportModel>;
