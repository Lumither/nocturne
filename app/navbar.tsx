'use client';

import { useEffect, useState } from 'react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from '@/components/ui/navigation-menu';
import Link from 'next/link';

const components: { title: string; href: string; description: string }[] = [
    {
        title: 'title of subpage',
        href: 'url',
        description:
            'a description'
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
                : 'w-full border-slate-200/15 border-b shadow' }` }>
            <div className={ `transition-all duration-300 ${ scrolled ? 'w-0' : 'w-[5%]' }` }></div>
            <NavigationMenu className={ `transition-all duration-300 ${ scrolled ? '' : 'my-5' }` }>
                <NavigationMenuList
                    className={ `flex space-x-4  transition-all duration-500 ${ scrolled ? 'space-x-3' : '' }` }>
                    <NavigationMenuItem>
                        <Avatar className={ 'transition-all duration-300 hover:scale-125' }>
                            <AvatarImage src={ 'https://avatars.githubusercontent.com/u/46409277' } />
                            <AvatarFallback>Tao</AvatarFallback>
                        </Avatar>

                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <Link href={ '/' }>
                            <p className={ 'text-xl transition-all duration-300 hover:text-resolution-blue-400' }>{ 'Lumither\'s Blog' }</p>
                        </Link>
                    </NavigationMenuItem>

                    <div></div>

                    <NavigationMenuItem>
                        <Link href={ '/about' }>
                            <p className={ 'transition-all duration-300 hover:text-resolution-blue-400' }>{ 'About' }</p>
                        </Link>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <Link href={ '/archives' }>
                            <p className={ 'transition-all duration-300 hover:text-resolution-blue-400' }>{ 'Archives' }</p>
                        </Link>
                    </NavigationMenuItem>

                </NavigationMenuList>
            </NavigationMenu>

        </div>
    </>;
}
