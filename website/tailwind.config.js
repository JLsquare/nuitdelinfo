/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./*.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        title: ['"Aquire"', 'sans-serif'],
      },
      colors: {
        primary: {
          DEFAULT: '#52B788',
        }
      }
    }
  },
  plugins: [],
}

