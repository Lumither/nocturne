import React from 'react';

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
