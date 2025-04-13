import React from 'react';
import { Metadata } from 'next';
import { SITE_CONFIG } from '@/src/constants';

export const metadata: Metadata = {
    title: `Friends - ${ SITE_CONFIG.name }`
};

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={ 'p-7' }>
            { children }
        </div>
    );
}

export default Layout;
