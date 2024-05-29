import { nextui } from '@nextui-org/react';

import type { Config } from 'tailwindcss';
import plugin from 'tailwindcss/plugin';

const config: Config = {
    content: [
        './pages/**/*.{js,ts,jsx,tsx,mdx}',
        './components/**/*.{js,ts,jsx,tsx,mdx}',
        './app/**/*.{js,ts,jsx,tsx,mdx}',
        './node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}'
    ],
    // theme: {
    //     extend: {}
    // },
    darkMode: 'class',
    plugins: [
        require('@tailwindcss/typography'),
        nextui(
            {
                // defaultTheme: 'dark',
                defaultExtendTheme: 'light',
                themes: {
                    'light': {
                        layout: {},
                        colors: {}
                    },
                    'dark': {
                        layout: {},
                        colors: {}
                    }
                }
            }
        ),
        plugin(function ({ addUtilities, addComponents, e, config }) {
            addUtilities(
                {
                    '.horizontal-tb': {
                        writingMode: 'horizontal-tb'
                    },
                    '.vertical-rl': {
                        writingMode: 'vertical-rl'
                    },
                    '.vertical-lr': {
                        writingMode: 'vertical-lr'
                    },
                    '.sideways-lr': {
                        writingMode: 'sideways-lr'
                    },
                    '.sideways-rl': {
                        writingMode: 'sideways-rl'
                    }
                }
            );
        })
    ]
};
export default config;
