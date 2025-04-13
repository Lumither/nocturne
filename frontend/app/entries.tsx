import React from 'react';
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

export const entries: { display_name: string; href: string; icon: React.ReactNode }[] = [
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

export const connections: { label: string, href: string, icon: React.ReactNode }[] = [
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
