'use client';

import React, { useEffect, useState } from 'react';
import { Button, Image } from '@nextui-org/react';
import NextImage from 'next/image';
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
import { usePathname } from 'next/navigation';

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
        href: '/archive'
    },
    {
        icon: <IoSearch size={ `20px` }></IoSearch>,
        display_name: 'Search',
        href: '/search'
    }
];

const connections: { label: string, href: string, icon: React.ReactNode }[] = [
    {
        label: 'Github',
        href: 'https://github.com/Lumither',
        icon: <IoLogoGithub size={ `30px` } />
    },
    {
        label: 'email',
        href: 'mailto:lumither@outlook.com',
        icon: <IoMail size={ `30px` } />
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


    const path = usePathname();
    const [ shownPath, setShownPath ] = useState(path);
    useEffect(() => {
        let trimmed_path = path.split(`/`)[1];
        if (trimmed_path === '') {
            setShownPath('HOME');
        } else {
            setShownPath(trimmed_path.toUpperCase());
        }
    }, [ path ]);


    const minMobileWidth: number = 768;
    const [ isMobile, setIsMobile ] = useState(false);
    useEffect(() => {
        const updateMobileWidth = () => {
            if (window.innerWidth <= minMobileWidth) {
                setIsMobile(true);
            } else {
                setIsMobile(false);
            }
        };
        updateMobileWidth();
        window.addEventListener('resize', updateMobileWidth);
        return () => {
            window.removeEventListener('resize', updateMobileWidth);
        };

    }, []);


    return (
        <>
            <div
                className={ `flex flex-col w-auto md:w-full min-w-[50px] max-w-[256px] mr-0 md:mr-2 ml-2 md:ml-8 min-h-dvh pt-8 sticky` }>
                <div className={ `fixed min-h-screen h-full` }>
                    <div className={ `hidden md:block` }>
                        <div className={ `flex justify-center items-center my-7` }>
                            <Image
                                as={ NextImage }
                                src={ 'https://oss.lumither.com/blog/pictures/compr/1717153492846703409_avatar.webp' }
                                alt={ 'avatar' }
                                width={ 200 }
                                height={ 200 }
                                className={ 'rounded-full justify-self-center' }
                                fetchPriority={ 'high' }
                                removeWrapper
                            ></Image>
                        </div>

                        <div>
                            <p className={ `font-bold text-2xl` }>Lumither Tao</p>
                            <p className={ `text-xl text-zinc-500 dark:text-zinc-400` }>Ad Astra</p>
                        </div>
                    </div>

                    <div>
                        <ul className={ `flex flex-col space-y-2 md:space-y-0 md:flex-row items-center my-4 md:space-x-2` }>
                            {
                                connections.map((conn, key) => (
                                    <li key={ key }>
                                        <Button as={ Link }
                                                href={ conn.href }
                                                aria-label={ conn.label }
                                                variant={ `light` }
                                                color={ `default` }
                                                className={ `w-fit` }
                                                isIconOnly
                                        >
                                            { conn.icon }
                                        </Button>
                                    </li>
                                ))

                            }
                        </ul>
                    </div>

                    <div
                        className={ `flex justify-center md:justify-start w-full mt-8 ml-0 md:-ml-3` }>
                        <ul className={ `flex flex-col space-y-2 w-full` }>
                            {
                                entries.map((meta, key) => (
                                    <li key={ key }>
                                        <Button
                                            as={ Link }
                                            variant={ `light` }
                                            color={ `default` }
                                            fullWidth
                                            isIconOnly={ isMobile }
                                            className={ 'flex md:justify-start w-fit md:w-full' }
                                            aria-label={ `navbar: ${ meta.display_name }` }
                                            href={ meta.href }>
                                            { meta.icon }
                                            <p className={ `font-bold hidden md:block` }>{ meta.display_name }</p>
                                        </Button>
                                    </li>
                                ))
                            }
                        </ul>
                    </div>

                    <div className={ `mt-8 flex md:hidden justify-center w-full` }>
                        <p className={ `text-center sideways-lr font-bold text-2xl text-zinc-500 dark:text-zinc-400` }>
                            { shownPath }
                        </p>
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
