import React from 'react';
import Markdown from 'react-markdown';
import { Card, CardBody } from '@nextui-org/card';
import remarkGfm from 'remark-gfm';
import remarkToc from 'remark-toc';
import remarkFrontmatter from 'remark-frontmatter';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import rehypeRaw from 'rehype-raw';
import rehypeSanitize from 'rehype-sanitize';
import Headers from '@/app/blog/post/[id]/Headers';
import Footer from '@/app/blog/post/[id]/Footer';
import { fetchNocturneApi } from '@/app/blog/api';

type Props = {
    id: string
}

async function PostCard(props: Props) {


    try {
        const res = await fetchNocturneApi(`/get_post/${ props.id }`);
        const ret: any = await res.json();
        return (
            <div>
                <Card className={ `max-w-full relative` }>

                    {/* card headers */ }
                    <Headers post={ ret } />

                    {/* card body */ }
                    <CardBody>
                        <Markdown
                            // className={ `p-5 max-w-full text-justify text-pretty prose prose-neutral prose-sm dark:prose-invert md:prose-md lg:prose-lg` }
                            className={ `p-5 max-w-full text-justify text-pretty prose prose-sm prose-neutral dark:prose-invert md:prose-md lg:prose-lg
                            prose-code:overflow-x-scroll
                        ` }
                            remarkPlugins={ [ remarkGfm, remarkToc, remarkFrontmatter ] }
                            rehypePlugins={ [ rehypeRaw, rehypeSanitize ] }
                            components={ {
                                pre(props) {
                                    const { node, className, children, ...rest } = props;
                                    if ((children as any)['type'] === 'code') {
                                        const match = /language-(\w+)/.exec((children as any)['props']['className'] || '');
                                        let lang = match ? match[1] : 'text';
                                        return (
                                            <pre className={ `not-prose` }>
                                            <SyntaxHighlighter
                                                // @ts-ignore
                                                style={ oneDark }
                                                language={ lang }
                                                showLineNumbers
                                                // wrapLongLines
                                                // wrapLines
                                                classNames={ className }
                                                PreTag="div"
                                                { ...rest }
                                            >
                                                {
                                                    String((children as any)['props']['children']).replace(/\n$/, '')
                                                }
                                            </SyntaxHighlighter>
                                        </pre>
                                        );
                                    } else {
                                        return (
                                            <pre className={ `not-prose` }>
                                            <code className={ className } { ...props }>
                                                { children }
                                            </code>
                                        </pre>
                                        );
                                    }
                                }
                            } }
                        >{ ret['content'] }</Markdown>
                    </CardBody>


                    {/* card footer */ }

                    <Footer post={ ret } />

                </Card>
            </div>
        );
    } catch (e: any) {
        return <div>
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: { e.toString() }
                </CardBody>
            </Card>
        </div>;
    }

}

export default PostCard;
