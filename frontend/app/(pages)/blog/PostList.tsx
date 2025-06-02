'use server';

import React from 'react';
import BlogPostCard from '@/app/(pages)/blog/BlogPostCard';

import * as motion from 'motion/react-client';
import { fetchPostList } from '@/src/api/blog/post';

const PostList = async ({ page }: { page: number }) => {
    try {
        const postListResponse = await fetchPostList(page);

        const posts = postListResponse.data.posts;
        return (
            // blog list
            <div className={ 'w-full' }>
                <ul className={ 'flex flex-col items-center w-full space-y-7' }>
                    { posts.map((post, key: number) => (
                        <li key={ key } className={ 'w-full' }>
                            <motion.div
                                initial={ { y: 20, opacity: 0 } }
                                animate={ { y: 0, opacity: 1 } }
                                transition={ { ease: 'easeInOut', duration: 0.5, delay: key * .2 + .25 } }
                            >
                                <BlogPostCard post={ post } />
                            </motion.div>
                        </li>)
                    ) }
                </ul>

            </div>
        );
    } catch (e: any) {
        return (
            <>
                <div className={ `flex h-full min-h-dvh w-full justify-center items-center` }>
                    <div className={ `flex-row` }>
                        <p className={ `text-xl font-bold` }>
                            { 'Unexpected Error:' }
                        </p>
                        <p className={ `font-bold` }>
                            { e.message }
                        </p>
                    </div>
                </div>
            </>
        );

    }
};
export default PostList;
