/** @type {import('tailwindcss').Config} */
export default {
  purge: ['./index.html', './src/**/*.{vue, js, ts, jsx, tsx'],
  content: ['./src/components/*.{vue,js,ts}',
            './src/App.vue',
            './src/pages/*.{vue,js,ts}'],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}

