import React from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkToc from 'remark-toc';
import remarkFrontmatter from 'remark-frontmatter';
import rehypeRaw from 'rehype-raw';
import rehypeSanitize from 'rehype-sanitize';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';

const MarkdownRenderer = ({ children }: { children: React.ReactNode }) => {
    return (
        <div className={ 'w-full' }>
            <Markdown
                className={ `max-w-full text-justify text-pretty prose prose-sm prose-neutral dark:prose-invert
                 md:prose-md lg:prose-lg prose-code:overflow-x-scroll`
                }
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
            >
                {
                    children as any
                }
            </Markdown>
        </div>
    );
};
export default MarkdownRenderer;
