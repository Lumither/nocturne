'use client';

import React from 'react';
import { Pagination as NextUIPagination } from '@nextui-org/react';
import { useRouter, useSearchParams } from 'next/navigation';
import { useScreenSizeTrigger } from '@/app/(pages)/public/screenSizeTrigger';

const Pagination = ({ total }: {
    total: number,
}) => {
    const parsePage = Number(useSearchParams().get('page'));
    const currPage = parsePage > 0 ? parsePage : 1;

    const router = useRouter();

    const isMobile = useScreenSizeTrigger('md');

    return (
        <div className={ 'w-full p-4 pt-6 flex justify-center' }>
            <NextUIPagination
                initialPage={ currPage }
                total={ total }
                siblings={ isMobile ? 0 : 1 }
                size={ isMobile ? 'sm' : 'md' }
                showControls
                showShadow
                color={ 'default' }
                variant={ 'light' }
                onChange={ (page) => {
                    router.push(`?page=${ page }`, { scroll: false });
                } }
            />
        </div>

    );
};
export default Pagination;
