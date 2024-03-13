'use client';

import { useEffect, useState } from 'react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';

export default function Header() {
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
        <div className={ `flex flex-row fixed w-full top-0 z-10 transition-all duration-300 ${ scrolled ? 'rounded-b-lg backdrop-blur' : 'bg-opacity-0' }` }>
            <Avatar>
                <AvatarImage src={ 'https://avatars.githubusercontent.com/u/46409277' } />
                <AvatarFallback>Tao</AvatarFallback>
            </Avatar>
            <p className={`text-xl align-middle`}>{ 'Lumither\'s Blog' }</p>

        </div>
    </>;
}
