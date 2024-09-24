import type { Config } from "tailwindcss";
import { addDynamicIconSelectors } from "@iconify/tailwind";
import typography from "@tailwindcss/typography";
import colors from "tailwindcss/colors";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      typography: () => ({
        mdxeditor: {
          css: {
            "--tw-prose-bullets": "rgb(0 0 0 / 50%)",
            "--tw-prose-quote-borders": "rgb(0 0 0 / 50%)",
            "--tw-prose-counters": "rgb(0 0 0 / 50%)",
            "--tw-prose-links": colors.purple[500],
          },
        },
      }),
    },
  },
  plugins: [typography(), addDynamicIconSelectors()],
};

export default config;
