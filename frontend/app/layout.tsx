import type { Metadata } from 'next';
import './globals.css';
import React from 'react';
import { ThemeProvider as NextThemesProvider } from 'next-themes';
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';


export const metadata: Metadata = {
    title: 'Lumitherの酒馆',
    description: '一间位于时间夹缝中的旅馆，静候下一位旅者的到来...'
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
            <div className={ `justify-center flex flex-row w-full max-w-full` }>
                <Navbar />
                <div className={ `flex flex-col flex-1 max-w-[1024px] min-w-0` }>
                    { children }
                    <Footer />
                </div>
            </div>
        </NextThemesProvider>
        </body>
        </html>
    );
}
