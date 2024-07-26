import React from 'react';

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        // <div className={ 'flex flex-auto p-7 pl-2 md:pl-7 items-stretch justify-center' }>
        <div className={ 'flex flex-auto p-7 pl-2 lg:pl-7 w-full justify-center' }>
            { children }
        </div>
    );
}

export default Layout;
