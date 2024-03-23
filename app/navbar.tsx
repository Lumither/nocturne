'use client';

import { useEffect, useState } from 'react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from '@/components/ui/navigation-menu';
import Link from 'next/link';

const entries: { display_name: string; href: string; }[] = [
    {
        display_name: 'About',
        href: '/about'
    },
    {
        display_name: 'Archive',
        href: '/archive'
    }

];
export default function Navbar() {
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

    return <>
        <div
            className={ `flex flex-row fixed top-0 z-10 transition-all duration-300 backdrop-blur 
            ${ scrolled ? 'rounded-3xl w-[95%] ml-[2.5%] mr-[2.5%] mt-[1%] border-resolution-blue-200 border'
                : 'w-full border-slate-200/15 border-b shadow h-20' }` }>
            <div className={ `transition-all duration-300 ${ scrolled ? 'w-0' : 'w-[5%]' }` }></div>
            <NavigationMenu className={ `transition-all duration-300` }>
                <NavigationMenuList
                    className={ `flex space-x-4  transition-all duration-500 ${ scrolled ? 'space-x-3' : '' }` }>
                    <NavigationMenuItem>

                        <Link href={ '/about' }>
                            <Avatar className={ 'transition-all duration-300 hover:scale-125' }>
                                <AvatarImage src={ 'https://avatars.githubusercontent.com/u/46409277' }
                                             alt={ 'Lumither\'s Avatar' } />
                                <AvatarFallback>Tao</AvatarFallback>
                            </Avatar>
                        </Link>

                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <Link href={ '/' }>
                            <p className={ 'text-xl transition-all duration-300 hover:text-resolution-blue-400' }>{ 'Lumither\'s Blog' }</p>
                        </Link>
                    </NavigationMenuItem>

                    <NavigationMenuItem></NavigationMenuItem>

                    {
                        entries.map((entry, id) => (
                            <NavigationMenuItem key={ id }>
                                <Link href={ entry.href }>
                                    <p className={ 'transition-all duration-300 hover:text-resolution-blue-400' }>{ entry.display_name }</p>
                                </Link>
                            </NavigationMenuItem>
                        ))
                    }

                </NavigationMenuList>
            </NavigationMenu>

        </div>
    </>;
}
