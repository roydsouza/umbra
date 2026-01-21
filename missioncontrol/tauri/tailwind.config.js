/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                bg: {
                    primary: '#0a0a1a',
                    secondary: '#12122a',
                    tertiary: '#1a1a3a',
                    card: 'rgba(26, 26, 58, 0.7)',
                },
                text: {
                    primary: '#ffffff',
                    secondary: '#a0a0c0',
                    muted: '#606080',
                },
                accent: {
                    blue: '#4a9eff',
                    purple: '#8b5cf6',
                    green: '#10b981',
                    red: '#ef4444',
                    yellow: '#f59e0b',
                    cyan: '#06b6d4',
                }
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
                mono: ['JetBrains Mono', 'monospace'],
            },
            backgroundImage: {
                'gradient-primary': 'linear-gradient(135deg, #4a9eff 0%, #8b5cf6 100%)',
                'gradient-success': 'linear-gradient(135deg, #10b981 0%, #06b6d4 100%)',
                'gradient-danger': 'linear-gradient(135deg, #ef4444 0%, #f59e0b 100%)',
            }
        },
    },
    plugins: [],
}
