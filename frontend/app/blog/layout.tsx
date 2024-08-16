import React from 'react';

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
