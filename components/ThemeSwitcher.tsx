'use client';

import React, { useEffect, useState } from 'react';
import { useTheme } from 'next-themes';
import { Button } from '@nextui-org/react';

function ThemeSwitcher() {
    const [ mounted, setMounted ] = useState(false);
    const { setTheme } = useTheme();

    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) {
        return null;
    }

    return (
        <div>
            <Button onClick={ () => {
                setTheme('light');
            } }>Light Mode</Button>
            <Button onClick={ () => {
                setTheme('dark');
            } }>Dark Mode</Button>
        </div>
    );
}

export default ThemeSwitcher;
