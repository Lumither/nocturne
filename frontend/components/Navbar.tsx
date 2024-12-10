'use client';

import React, { useEffect, useState } from 'react';
import { Button } from '@nextui-org/react';
import Image from 'next/image';
import {
    IoFlask,
    IoHome,
    IoInformationCircle,
    IoLink,
    IoLogoGithub,
    IoMail,
    IoPencil,
    IoSearch
} from 'react-icons/io5';
import Link from 'next/link';
import ThemeSwitcher from '@/components/ThemeSwitcher';
import { usePathname } from 'next/navigation';
import { useScreenSizeTrigger } from '@/app/public/screenSizeTrigger';

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
        icon: <IoLink size={ `20px` }></IoLink>,
        display_name: 'Friends',
        href: '/friends'
    },
    {
        icon: <IoSearch size={ `20px` }></IoSearch>,
        display_name: 'Search',
        href: '/search'
    }, {
        icon: <IoFlask size={ '20px' } />,
        display_name: 'Lab',
        href: 'https://lab.lmt.moe'
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

    const isMobile = useScreenSizeTrigger('lg');

    return (
        <>
            <div
                className={ `flex flex-col min-w-[50px] lg:min-w-[200px] ml-2 mr-0 lg:ml-8 lg:mr-6 min-h-screen pt-8 sticky` }>
                <div className={ `fixed min-h-screen h-full` }>
                    <div className={ `hidden lg:block` }>
                        <div className={ `flex justify-center items-center my-7 h-[200px] w-[200px]` }>
                            <div className={ 'relative w-full h-full' }>
                                <Image
                                    src={ 'https://oss.lumither.com/blog/pictures/avatar.webp' }
                                    alt={ 'avatar' }
                                    fill
                                    className={ 'rounded-full object-cover' }
                                    loading={ 'eager' }
                                ></Image>
                            </div>
                        </div>

                        <div>
                            <p className={ `font-bold text-2xl` }>Lumither Tao</p>
                            <p className={ `text-xl text-zinc-500 dark:text-zinc-400` }>Ad Astra</p>
                        </div>
                    </div>

                    <div>
                        <ul className={ `flex flex-col space-y-2 lg:space-y-0 lg:flex-row items-center my-4 lg:space-x-2` }>
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
                        className={ `flex justify-center lg:justify-start w-full mt-8 ml-0 lg:-ml-3` }>
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
                                            className={ 'flex lg:justify-start w-fit lg:w-full' }
                                            aria-label={ `navbar: ${ meta.display_name }` }
                                            href={ meta.href }
                                        >
                                            { meta.icon }
                                            <p className={ `font-bold hidden lg:block` }>{ meta.display_name }</p>
                                        </Button>
                                    </li>
                                ))
                            }
                        </ul>
                    </div>

                    <div className={ `mt-8 flex lg:hidden justify-center w-full` }>
                        <p className={ `text-center sideways-lr font-bold text-2xl text-zinc-500 dark:text-zinc-400` }>
                            { shownPath }
                        </p>
                    </div>

                    <div
                        className={ `absolute bottom-16` }
                    >
                        <ThemeSwitcher />
                    </div>
                </div>
            </div>
        </>
    );
}

export default Navbar;
