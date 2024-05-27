'use client';

import React, { useEffect, useState } from 'react';
import { Button, Image } from '@nextui-org/react';
import {
    IoFileTrayFull,
    IoHome,
    IoInformationCircle,
    IoLogoGithub,
    IoMail,
    IoPencil,
    IoPricetags,
    IoSearch
} from 'react-icons/io5';
import Link from 'next/link';
import ThemeSwitcher from '@/components/ThemeSwitcher';

const entries: { display_name: string; href: string; icon: React.ReactNode }[] = [
    {
        icon: <IoHome size={ `20px` }></IoHome>,
        display_name: 'Home',
        href: '/'
    },
    {
        icon: <IoInformationCircle size={ `20px` }></IoInformationCircle>,
        display_name: 'About',
        href: '/about'
    },
    {
        icon: <IoPencil size={ `20px` }></IoPencil>,
        display_name: 'Blog',
        href: '/blog'
    },
    {
        icon: <IoPricetags size={ `20px` }></IoPricetags>,
        display_name: 'Tags',
        href: '/tags'
    },
    {
        icon: <IoFileTrayFull size={ `20px` }></IoFileTrayFull>,
        display_name: 'Archive',
        href: '/Archive'
    },
    {
        icon: <IoSearch size={ `20px` }></IoSearch>,
        display_name: 'Search',
        href: '/search'
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
            <div
                className={ `flex flex-col w-full max-w-[256px] mr-2 ml-8 min-h-screen max-h-screen pt-8 sticky` }>
                <div className={ `fixed min-h-screen h-full` }>
                    <div className={ `flex justify-center items-center my-7` }>
                        <Image
                            src={ 'https://avatars.githubusercontent.com/u/46409277?v=4' }
                            alt={ 'avatar' }
                            width={ 200 }
                            height={ 200 }
                            className={ 'rounded-full justify-self-center' }
                            removeWrapper
                        ></Image>
                    </div>

                    <p className={ `font-bold text-2xl` }>Lumither Tao</p>
                    <p className={ `text-xl text-zinc-500 dark:text-zinc-400` }>Ad Astra</p>

                    <div className={ `flex flex-row items-center my-4 space-x-2` }>
                        <Link href={ 'https://github.com/Lumither' } aria-label={ `GitHub` }>
                            <IoLogoGithub size={ `30px` } />
                        </Link>
                        <Link href={ 'mailto:lumither@outlook.com' } aria-label={ `email` }>
                            <IoMail size={ `30px` } />
                        </Link>
                    </div>

                    <div
                        className={ `mt-8 -ml-3` }>
                        <ul className={ `flex flex-col space-y-2` }>
                            {
                                entries.map((meta, key) => (
                                    <li key={ key }>
                                        <Button startContent={ meta.icon }
                                                as={ Link }
                                                variant={ `light` }
                                                color={ `default` }
                                                fullWidth
                                                className={ 'flex justify-start' }
                                            // className={ `w-full` }
                                                aria-label={ `navbar: ${ meta.display_name }` }
                                                href={ meta.href }>
                                            <p className={ `font-bold` }>{ meta.display_name }</p>
                                        </Button>
                                    </li>
                                ))
                            }
                        </ul>
                    </div>

                    <div
                        className={ `absolute bottom-8 mb-8` }
                    >
                        <ThemeSwitcher />
                    </div>
                </div>
            </div>
        </>
    );
}

export default Navbar;
