'use client';

import React from 'react';
import BlogPostCard from '@/app/(pages)/blog/BlogPostCard';

import * as motion from 'motion/react-client';
import { AnimatePresence } from 'motion/react';
import { Post } from '@/src/api/blog/post';

export interface HighlightSelector {
    tags?: string[];
}

interface Props {
    posts: Post[];
    highlight: HighlightSelector;
}

const PostList = ({ posts, highlight }: Props) => {
    return (
        <div className={ 'w-full' }>
            <ul className={ 'flex flex-col items-center w-full space-y-7' }>
                <AnimatePresence mode={ 'wait' }>
                    { posts.map((post, key: number) => (
                        <motion.li
                            initial={ { y: 20, opacity: 0 } }
                            animate={ { y: 0, opacity: 1 } }
                            transition={ { ease: 'easeOut', duration: 0.5, delay: key * .1 + .1 } }
                            key={ key }
                            className={ 'w-full' }>
                            <BlogPostCard post={ post } highlightSelector={ highlight } />
                        </motion.li>)
                    ) }
                </AnimatePresence>
            </ul>

        </div>
    );
};
export default PostList;
