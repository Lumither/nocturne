import React from 'react';
import { Metadata } from 'next';
import { SITE_CONFIG } from '@/src/constants';

export const metadata: Metadata = {
    title: `About - ${ SITE_CONFIG.name }`
};

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={ 'flex p-7' }>
            { children }
        </div>
    );
}

export default Layout;
