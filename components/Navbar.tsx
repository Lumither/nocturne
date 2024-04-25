'use client';

import React, { useEffect, useState } from 'react';
import {
    Image,
    Navbar as NextUINavbar,
    NavbarBrand,
    NavbarContent,
    NavbarItem,
    NavbarMenu,
    NavbarMenuItem,
    NavbarMenuToggle
} from '@nextui-org/react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import ThemeSwitcher from '@/components/ThemeSwitcher';


const entries: { display_name: string; href: string; }[] = [
    {
        display_name: 'Home',
        href: '/'
    },
    {
        display_name: 'About',
        href: '/about'
    },
    {
        display_name: 'Blog',
        href: '/blog'
    }
];

function Navbar() {
    const [ scrolled, setScrolled ] = useState(false);

    useEffect(() => {
        const handleScroll = () => {
            const isScrolled = window.scrollY > 0;
            setScrolled(isScrolled);
        };
        window.addEventListener('scroll', handleScroll);
        return () => {
            window.removeEventListener('scroll', handleScroll);
        };
    }, []);


    return (
        <>
            <NextUINavbar isBlurred isBordered>

                <NavbarContent className="sm:hidden" justify="start">
                    <NavbarMenuToggle />
                </NavbarContent>

                <NavbarContent className="sm:hidden pr-3" justify="center">
                    <Image
                        src={ 'https://avatars.githubusercontent.com/u/46409277?v=4' }
                        alt={ 'avatar' }
                        className={ 'rounded-full h-[3rem]' }
                    ></Image>

                    <NavbarBrand>
                        <p className="font-bold text-inherit">{ 'Lumither\'s site' }</p>
                    </NavbarBrand>
                </NavbarContent>

                <NavbarContent className="hidden sm:flex gap-4" justify="center">
                    <Image
                        src={ 'https://avatars.githubusercontent.com/u/46409277?v=4' }
                        alt={ 'avatar' }
                        className={ 'rounded-full h-[3rem]' }
                    ></Image>

                    <NavbarBrand>
                        <p className="font-bold text-inherit">{ 'Lumither\'s site' }</p>
                    </NavbarBrand>

                    { entries.map((item, index) => (
                        // eslint-disable-next-line react-hooks/rules-of-hooks
                        <NavbarItem key={ index } isActive={ usePathname() === item.href }>
                            <Link
                                className="w-full"
                                href={ item.href }
                            >
                                { item.display_name }
                            </Link>
                        </NavbarItem>
                    )) }
                </NavbarContent>

                <NavbarContent justify="end">
                    <NavbarItem>
                        <ThemeSwitcher />
                    </NavbarItem>
                </NavbarContent>

                <NavbarMenu>
                    { entries.map((item, index) => (
                        <NavbarMenuItem key={ index }>
                            <Link
                                className="w-full"
                                href={ item.href }
                            >
                                { item.display_name }
                            </Link>
                        </NavbarMenuItem>
                    )) }
                </NavbarMenu>
            </NextUINavbar>
        </>
    );
}

export default Navbar;
