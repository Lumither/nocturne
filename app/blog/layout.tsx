import React from 'react';

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={ 'flex items-stretch justify-center' }>
            { children }
        </div>
    );
}

export default Layout;
