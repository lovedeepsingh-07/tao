import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import checker from "vite-plugin-checker";

export default defineConfig({
	plugins: [checker({ typescript: true }), tailwindcss(), sveltekit()],
	server: {
		port: 3000,
		host: true
	}
});
