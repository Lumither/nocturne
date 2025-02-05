import React from 'react';
import { Metadata } from 'next';
import { SITE_CONFIG } from '@/src/constants';

export const metadata: Metadata = {
    title: `Blog - ${ SITE_CONFIG.name }`
};

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={ 'flex flex-auto p-7 pl-2 lg:pl-7 w-full justify-center' }>
            <main className={ 'w-full' }>
                { children }
            </main>
        </div>
    );
}

export default Layout;
