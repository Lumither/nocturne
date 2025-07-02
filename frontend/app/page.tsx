'use client';

import React from 'react';
import { useScreenSizeTrigger } from '@/app/(pages)/public/screenSizeTrigger';
import { Button } from '@nextui-org/react';
import { entries } from '@/app/entries';
import Link from 'next/link';

export default function Home() {

    const isMobile = useScreenSizeTrigger('lg');

    return (
        <div>
            <div className={ `min-h-dvh h-min content-center` }>
                <p className={ 'text-center text-4xl' }>Atelier Lumiaethor</p>
            </div>

            <div
                // className={ 'absolute bottom-0 w-full h-60 bg-yellow-500 p-12' }
                className={ 'absolute bottom-0 w-full h-60 p-12' }
            >
                <div
                    className={ 'bg-blue-500 h-full w-full rounded-3xl backdrop-blur align-middle grid grid-cols-2 content-center' }>

                    <div className={ 'w-full flex justify-between' }>
                        <div className={ 'p-16' }>
                            test
                        </div>

                        <div className={ 'w-0.5 h-full bg-neutral-400' }></div>
                    </div>

                    <div className={ 'h-full content-center p-4 pr-3' }>
                        <ul className={ `grid grid-cols-2 w-full` }>
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
                </div>
            </div>
        </div>
    );
}
