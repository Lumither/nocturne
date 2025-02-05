import { useEffect, useState } from 'react';

const screenSize = new Map<string, number>(
    [
        [ 'sm', 640 ],
        [ 'md', 768 ],
        [ 'lg', 1024 ],
        [ 'xl', 1280 ],
        [ '2xl', 2536 ]
    ]
);

export const useScreenSizeTrigger = (size: number | string) => {
    const criticalSize = (typeof size === 'string') ? (screenSize.get(size) ?? 768) : size;

    const [ isCritical, setIsCritical ] = useState(false);
    useEffect(() => {
        const updateMobileWidth = () => {
            if (window.innerWidth < criticalSize) {
                setIsCritical(true);
            } else {
                setIsCritical(false);
            }
        };
        updateMobileWidth();
        window.addEventListener('resize', updateMobileWidth);
        return () => {
            window.removeEventListener('resize', updateMobileWidth);
        };

    });

    return isCritical;

};
