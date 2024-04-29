import { nextui } from '@nextui-org/react';

import type { Config } from 'tailwindcss';

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
    ) ]
};
export default config;
