'use client';

import { useEffect, useState } from 'react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { NavigationMenu, NavigationMenuItem, NavigationMenuList } from '@/components/ui/navigation-menu';

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
                : 'w-full border-slate-200/15 border-b shadow h-' }` }>
            <div className={ `transition-all duration-300 ${ scrolled ? 'w-0' : 'w-[5%]' }` }></div>
            <NavigationMenu>
                <NavigationMenuList>
                    <NavigationMenuItem>
                        <Avatar>
                            <AvatarImage src={ 'https://avatars.githubusercontent.com/u/46409277' } />
                            <AvatarFallback>Tao</AvatarFallback>
                        </Avatar>

                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <p className={ 'text-xl' }>{"Lumither\'s Blog"}</p>
                    </NavigationMenuItem>

                </NavigationMenuList>
            </NavigationMenu>

        </div>
    </>;
}
