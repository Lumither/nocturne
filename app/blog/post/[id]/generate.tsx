'use server';

import React from 'react';
import Markdown from 'react-markdown';
import { Card, CardBody } from '@nextui-org/card';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import rehypeSanitize from 'rehype-sanitize';
import rehypeRaw from 'rehype-raw';
import remarkToc from 'remark-toc';

type Props = {
    id: string
}

async function Generate(props: Props) {
    const response = await fetch(`${ process.env.BLOG_ENDPOINT }/posts/${ props.id }`);
    if (!response.ok) {
        throw new Error(response.statusText);
    }
    const post_md = await response.text();

    console.log(post_md);

    return (
        <div>
            <Card className={ `w-full max-w-[1024px]` }>
                <CardBody>
                    <Markdown
                        className={ `mx-5 my-5 max-w-none prose dark:prose-invert sm:prose-sm md:prose-sm lg:prose-lg` }
                        remarkPlugins={ [ remarkGfm, remarkToc ] }
                        rehypePlugins={ [ rehypeHighlight, rehypeRaw, rehypeSanitize ] }
                    >{ post_md }</Markdown>
                </CardBody>

            </Card>
        </div>
    );
}

export default Generate;
