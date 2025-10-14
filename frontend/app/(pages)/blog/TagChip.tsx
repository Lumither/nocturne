'use client';

import React from 'react';
import { Chip } from '@nextui-org/react';
import { useRouter, useSearchParams } from 'next/navigation';

interface TagChipProps {
    tagName: string,
    isHighlighted: boolean
}

const TagChip = ({ tagName, isHighlighted }: TagChipProps) => {
    const router = useRouter();
    const searchParams = useSearchParams();

    const toggleTag = (e: React.MouseEvent<HTMLDivElement>) => {
        e.stopPropagation();
        e.preventDefault();

        const params = new URLSearchParams(searchParams.toString());
        const tags = params.getAll('tags');

        if (tags.includes(tagName)) {
            const newTags = tags.filter(t => t !== tagName);
            params.delete('tags');
            newTags.forEach(t => params.append('tags', t));
        } else {
            params.append('tags', tagName);
        }

        const queryString = params.toString();
        router.push(`?${ queryString }`, { scroll: false });
    };
    return (
        <div
            onClick={ toggleTag }
            className="cursor-pointer relative"
        >
            <Chip
                className={ 'hover:border-amber-400 hover:dark:border-amber-700  transition-all' }
                variant={ 'bordered' }
                color={ isHighlighted ? 'warning' : 'default' }
            >{ tagName }</Chip>
        </div>
    );
};

export default TagChip;