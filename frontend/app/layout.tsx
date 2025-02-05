import type { Metadata } from 'next';
import './globals.css';
import React from 'react';
import { ThemeProvider as NextThemesProvider } from 'next-themes';
import { SITE_CONFIG } from '@/src/constants';

export const metadata: Metadata = {
    openGraph: {
        siteName: SITE_CONFIG.name
    },
    title: SITE_CONFIG.name,
    description: SITE_CONFIG.desc
};

export default function RootLayout({
    children
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" suppressHydrationWarning>
        <body className={ `transition duration-300 dark:bg-[#282830] min-h-screen` }>
        <NextThemesProvider attribute="class" defaultTheme="system">
            { children }
        </NextThemesProvider>
        </body>
        </html>
    );
}
