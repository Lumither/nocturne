'use client';

import React from 'react';
import { Chip } from '@nextui-org/react';
import { useRouter } from 'next/navigation';

const TagChip = ({ tag }: { tag: string }) => {
    const router = useRouter();

    return (
        <div onClick={ () => router.push(`/blog?tags=${ tag }`) }>
            <Chip
                className={ 'hover:border-amber-400 hover:dark:border-amber-700  transition-all' }
                variant={ 'faded' }
            >{ tag }</Chip>
        </div>
    );
};

export default TagChip;