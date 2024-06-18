import React from 'react';
import Link from 'next/link';
import { IoLogoGithub } from 'react-icons/io5';

const Footer = () => {
    return (
        <div className={ `p-5 mb-4` }>
            <div className={ `h-1 w-20 bg-zinc-500 dark:bg-zinc-400 mb-3` }></div>
            <p>
                © 2024-{ new Date().getFullYear() } <Link href={ `/about` }>Lumither Tao</Link>
            </p>
            <p>
                Powered by Next.js and Rust, built with passion and love
            </p>
            <Link
                href={ `https://github.com/Lumither/blog` }
                aria-label={ `GitHub repo for this website` }
            >
                <IoLogoGithub className={ `mt-1` } />
            </Link>
        </div>
    );
};

export default Footer;
