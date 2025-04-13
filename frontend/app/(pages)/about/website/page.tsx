import React from 'react';
import MarkdownRenderer from '@/app/(pages)/public/MarkdownRenderer';
import { fetchText } from '@/app/fetch';

const Website = async () => {
    return (
        <div className={ 'p-5' }>
            <MarkdownRenderer>
                { await fetchText('https://raw.githubusercontent.com/lumither/blog-posts/master/about/website.md') }
            </MarkdownRenderer>
        </div>
    );
};
export default Website;
