'use client';

import React, { useEffect, useState } from 'react';
import { Pagination as NextUIPagination } from '@nextui-org/react';
import { useRouter, useSearchParams } from 'next/navigation';

const Pagination = ({ total }: {
    total: number,
}) => {
    const parsePage = Number(useSearchParams().get('page'));
    const currPage = parsePage > 0 ? parsePage : 1;

    const router = useRouter();

    const maxMobileWidth: number = 768;
    const [ isMobile, setIsMobile ] = useState(false);
    useEffect(() => {
        const updateMobileWidth = () => {
            if (window.innerWidth <= maxMobileWidth) {
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
