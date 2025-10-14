'use client';

import React from 'react';
import { useScreenSizeTrigger } from '@/app/(pages)/public/screenSizeTrigger';
import { Button } from '@nextui-org/react';
import { entries } from '@/app/entries';
import Link from 'next/link';
import * as motion from 'motion/react-client';
import { cn } from '@/src/utils';

export default function Home() {

    const isSm = useScreenSizeTrigger('sm');
    const isXs = useScreenSizeTrigger('xs');

    return (
        <div className={ 'h-dvh w-full overflow-hidden relative' }>
            <motion.div
                initial={ { opacity: 0 } }
                animate={ { opacity: 1 } }
                transition={ { ease: 'easeInOut', duration: 0.3, delay: 0.0 } }
            >
                <div className={ `min-h-dvh h-min content-center z-20 p-10` }>
                    <p className={ 'text-center text-4xl' }>Atelier Lumiaethor</p>
                    <p className={ 'text-center text-gray-700 font-bold dark:text-gray-400' }>In pursuit of absolute
                        simplicity and ultimate
                        sophistication</p>
                </div>

                <div
                    className={ 'absolute left-1/2 -translate-x-1/2 bottom-0 w-full h-60 p-12 max-w-[1536px] justify-center' }
                >
                    <div
                        className={ 'h-full w-full rounded-3xl backdrop-blur align-middle grid grid-cols-1 sm:grid-cols-2 content-center' }>

                        { !isSm &&
                            <div className={ 'w-full grid content-center' }>
                                <div
                                    className={ 'p-2 h-fit brightness-50 lg:brightness-75 lg:p-8 xl:p-16 transition-all' }>
                                    <p>
                                        © 2024-{ new Date().getFullYear() } <Link href={ `/about` }>Lumither Tao</Link>
                                    </p>
                                    <p>Powered by Next.js and Rust, built with passion and love</p>
                                    <p>LMTCloud ecosystem</p>
                                </div>
                            </div>
                        }

                        <div className={ 'h-full content-center p-4' }>
                            <ul className={ `grid grid-cols-3 sm:grid-cols-2 justify-items-center sm:justify-items-start w-full` }>
                                {
                                    entries.map((meta, key) => (
                                        <li key={ key } className={ cn(isXs && 'w-fit', !isXs && 'w-full') }>
                                            <Button
                                                as={ Link }
                                                variant={ `light` }
                                                color={ `default` }
                                                fullWidth
                                                isIconOnly={ isXs }
                                                className={ cn(
                                                    'flex justify-center',
                                                    isXs && 'w-fit',
                                                    !isXs && 'sm:justify-start'
                                                ) }
                                                aria-label={ `navbar: ${ meta.display_name }` }
                                                href={ meta.href }
                                            >
                                                { meta.icon }
                                                { !isXs && <p className={ `font-bold` }>{ meta.display_name }</p> }
                                            </Button>
                                        </li>
                                    ))
                                }
                            </ul>
                        </div>


                    </div>
                </div>
            </motion.div>
        </div>
    );
}
