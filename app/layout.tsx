import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import React from 'react';
import { ThemeProvider as NextThemesProvider } from 'next-themes';
import Navbar from '@/components/Navbar';

const inter = Inter({ subsets: [ 'latin' ] });

export const metadata: Metadata = {
    title: 'Lumither\'s site',
    description: 'a small corner on internet'
};

export default function RootLayout({
    children
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" suppressHydrationWarning>
        <body className={ inter.className }>
        <NextThemesProvider attribute="class" defaultTheme="dark">
            <Navbar />
            { children }
        </NextThemesProvider>
        </body>
        </html>
    );
}
