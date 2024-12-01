'use client';

import React, { useEffect, useState } from 'react';
import { useTheme } from 'next-themes';
import { Button, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger } from '@nextui-org/react';
import { MdOutlineDarkMode, MdOutlineDesktopWindows, MdOutlineLightMode } from 'react-icons/md';
import { useScreenSizeTrigger } from '@/app/public/screenSizeTrigger';

const switcherElements = new Map(
    [
        [ 'dark', {
            'display_name': 'Dark Mode',
            'icon': <MdOutlineDarkMode />
        } ],
        [ 'light', {
            'display_name': 'Light Mode',
            'icon': <MdOutlineLightMode />
        } ],
        [ 'system', {
            'display_name': 'Follow System',
            'icon': <MdOutlineDesktopWindows />
        } ]
    ]
);

function ThemeSwitcher() {
    const [ mounted, setMounted ] = useState(false);
    const { theme, setTheme } = useTheme();

    const [ currTheme, setCurrTheme ] = useState(new Set([ theme as string ]));

    const isMobile = useScreenSizeTrigger('lg');

    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) {
        return null;
    }

    return (
        <div className={ `` }>
            <Dropdown backdrop={ 'blur' }>
                <DropdownTrigger>
                    <Button
                        variant={ 'flat' }
                        aria-label={ 'Theme Trigger' }
                        fullWidth
                        isIconOnly={ isMobile }
                    >
                        { switcherElements.get(currTheme.values().next().value as string)?.icon }
                        <p className={ 'hidden lg:block' }>
                            { switcherElements.get(currTheme.values().next().value as string)?.display_name }
                        </p>
                    </Button>
                </DropdownTrigger>

                <DropdownMenu
                    aria-label={ 'Toggle Theme' }
                    variant={ 'flat' }
                    selectionMode={ 'single' }
                    selectedKeys={ currTheme }
                    onSelectionChange={ ({ anchorKey }) => {
                        setCurrTheme(new Set([ anchorKey as string ]));
                        setTheme(anchorKey as string);
                    } }

                >
                    <DropdownItem
                        key={ 'light' }
                        startContent={ <MdOutlineLightMode /> }>
                        Light Mode
                    </DropdownItem>
                    <DropdownItem
                        key={ 'dark' }
                        startContent={ <MdOutlineDarkMode /> }
                    >
                        Dark Mode
                    </DropdownItem>
                    <DropdownItem
                        key={ 'system' }
                        startContent={ <MdOutlineDesktopWindows /> }
                    >
                        Follow System
                    </DropdownItem>
                </DropdownMenu>
            </Dropdown>
        </div>
    );
}

export default ThemeSwitcher;
