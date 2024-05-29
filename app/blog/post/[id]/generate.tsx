'use server';

import React from 'react';
import Markdown from 'react-markdown';
import { Card, CardBody } from '@nextui-org/card';
import remarkGfm from 'remark-gfm';
import rehypeHighlight from 'rehype-highlight';
import rehypeSanitize from 'rehype-sanitize';
import rehypeRaw from 'rehype-raw';
import remarkToc from 'remark-toc';
import remarkFrontmatter from 'remark-frontmatter';
import axios from 'axios';

type Props = {
    id: string
}

async function Generate(props: Props) {

    let response: any;
    try {
        response = await axios(`http://localhost:${ process.env.BACKEND_PORT }/api/get/post/${ props.id }`);
    } catch (e) {
        return (
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: Failed to load the post.
                </CardBody>
            </Card>
        );
    }


    let post_data = response.data;

    return (
        <div>
            <Card className={ `w-full max-w-full relative` }>
                <CardBody>
                    <Markdown
                        className={ `p-5 max-w-full text-justify text-pretty会 prose prose-sm dark:prose-invert md:prose-md lg:prose-lg` }
                        remarkPlugins={ [ remarkGfm, remarkToc, remarkFrontmatter ] }
                        rehypePlugins={ [ rehypeHighlight, rehypeRaw, rehypeSanitize ] }
                    >{ (post_data as any)['content'] }</Markdown>
                </CardBody>
            </Card>
        </div>
    );
}

export default Generate;
