import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';

export default {
    content: [
        './src/**/*.{html,js,svelte,ts}'
    ],
    theme: {
        extend: {
            typography: {
                DEFAULT: {
                    css: {
                        ul: {
                            listStyleType: 'disc',
                        },
                        ol: {
                            listStyleType: 'decimal',
                        },
                        'ul, ol': {
                            paddingLeft: '1.5em',
                        },
                    },
                },
            },
        },
    },
    plugins: [
        typography,
    ]
} satisfies Config