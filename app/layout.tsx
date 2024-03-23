import type { Metadata } from 'next';
import './globals.css';

import { Inter as FontSans } from 'next/font/google';
import { cn } from '@/lib/utils';
import React from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import Navbar from '@/app/navbar';


const fontSans = FontSans({
    subsets: [ 'latin' ],
    variable: '--font-sans'
});
export const metadata: Metadata = {
    title: 'Lumither\'s blog',
    description: 'a little blog site...'
};

export default function RootLayout({
    children
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
        <body className={ cn(
            'min-h-screen bg-background font-sans antialiased',
            fontSans.variable
        ) }>
        <ThemeProvider
            attribute="class"
            defaultTheme="dark"
            enableSystem
            disableTransitionOnChange
        >
            <Navbar />
            <div className={ 'h-screen' }>
                <div className={ 'mt-20 overflow-scroll' }>
                    { children }
                </div>
            </div>
        </ThemeProvider>
        </body>
        </html>
    );
}
