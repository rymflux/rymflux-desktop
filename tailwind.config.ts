import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/**/*.{html,svelte,ts,js}",
    "../rymflux-audiobook/src/**/*.{html,svelte,ts,js}",
  ],
} satisfies Config;
