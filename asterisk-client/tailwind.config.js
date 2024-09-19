// import { addIconSelectors } from "@iconify/tailwind";
import { iconsPlugin, getIconCollections } from "@egoist/tailwindcss-icons";

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        "primary-passive": "#B0A0FF",
        "primary-highlight": "#7B61FF",
        "primary-selected": "#4A3A99",
        "primary-active": "#7B61FF",
        "text-passive": "#5C586B",
        "text-highlight": "#3C384A",
        "text-selected": "#000000",
        "border-passive": "#E3DFF0",
        "border-highlight": "#C0BCD2",
        "border-selected": "#A8A3C1",
        "border-active": "#7B61FF",
        "bg-passive": "#FFFFFF",
        "bg-highlight": "#FBFAFF",
        "bg-selected": "#F3F1FF",
        "bg-active": "#F3F1FF",
      },
    },
  },
  plugins: [
    iconsPlugin({
      collections: getIconCollections(["carbon", "mdi"]),
    }),
  ],
};
