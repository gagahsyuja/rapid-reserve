/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                'everforest': {
                    black: '#333C43',
                    white: '#D3C6AA',
                    red: '#E67E80',
                    orange: '#E69875',
                    yellow: '#DBBC7F',
                    green: '#A7C080',
                    blue: '#7FBBB3',
                    aqua: '#83C092',
                    purple: '#D699B6'
                }
            },
            fontFamily: {
                oswald: "Oswald"
            }
        },
    },
    plugins: [],
}

