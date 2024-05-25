import type { Metadata } from 'next';
import './globals.css';
import React from 'react';
import { ThemeProvider as NextThemesProvider } from 'next-themes';
import Navbar from '@/components/Navbar';


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
        <body className={ `dark:bg-[#282830] min-h-screen` }>
        <NextThemesProvider attribute="class" defaultTheme="dark">
            <div className={ `justify-center flex flex-row w-full` }>
                <Navbar />
                <div className={ `flex-auto max-w-[1024px]` }>
                    { children }
                </div>
            </div>
        </NextThemesProvider>
        </body>
        </html>
    );
}
