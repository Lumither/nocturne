'use client';

import React from 'react';
import { useRouter } from 'next/navigation';
import { Button } from '@nextui-org/react';
import { IoArrowBackOutline } from 'react-icons/io5';

const ButtonGoBack = () => {
    const router = useRouter();

    return (
        <Button
            startContent={ <IoArrowBackOutline /> }
            variant={ 'light' }
            onClick={ () => router.back() }
        >
            { 'Back' }
        </Button>
    );
};
export default ButtonGoBack;
