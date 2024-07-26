'use client';

import React, { useEffect, useState } from 'react';
import { useTheme } from 'next-themes';
import { Switch } from '@nextui-org/react';
import { MdOutlineDarkMode, MdOutlineLightMode } from 'react-icons/md';

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
        <div className={ `` }>
            <div className={ `flex flex-row items-center` }>
                <Switch
                    color={ 'default' }
                    aria-label={ `theme switch` }
                    startContent={ <MdOutlineLightMode /> }
                    endContent={ <MdOutlineDarkMode /> }
                    onValueChange={ (isSelected) => {
                        if (isSelected) {
                            setTheme('light');
                        } else {
                            setTheme('dark');
                        }
                    } }
                ></Switch>
                <p className={ `font-bold hidden lg:block` }>{ `Toggle Theme` }</p>
            </div>
        </div>
    );
}

export default ThemeSwitcher;
