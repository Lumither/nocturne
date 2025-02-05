'use client';

import React from 'react';
import { useScreenSizeTrigger } from '@/app/(pages)/public/screenSizeTrigger';
import { Button } from '@nextui-org/react';
import { entries } from '@/app/entries';
import Link from 'next/link';
import { useLocalStorage } from 'react-use';

export default function Home() {

    const isMobile = useScreenSizeTrigger('lg');

    const [ test, setTest, removeTest ] = useLocalStorage('test', 'test_value');
    console.log(test);
    // setTest('test value 2');
    // console.log(test);
    // removeTest();
    // console.log(test);

    return (
        <div className={ `h-dvh relative` }>
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
            <div
                className={ 'absolute bottom-0 w-full h-60 bg-yellow-500 p-12' }
            >
                <div className={ 'bg-blue-500 h-full w-full rounded-3xl backdrop-blur' }></div>
            </div>
        </div>
    );
}
