export const content = [
    "./src/**/*.{astro,js,ts,jsx,tsx}"
];
export const theme = {
    extend: {
        fontFamily: {
            'jomolhari': ['"Jomolhari"', 'serif'],
        },
        colors: {
            background: "var(--color-background)",
            red: "var(--color-red)",
            lightRed: "var(--color-light-red)",
            salmon: "var(--color-salmon)",
            lightSalmon: "var(--color-light-salmon)",
            lightBrown: "var(--color-light-brown)",
            brown: "var(--color-brown)",
            smoothBrown: "var(--color-smooth-brown)",
            white: "var(--color-white)",
            black: "var(--color-black)",
        }
    }
};
export const plugins = [];