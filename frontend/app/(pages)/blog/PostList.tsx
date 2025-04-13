'use server';

import React from 'react';
import { fetchNocturneApi } from '@/app/(pages)/blog/api';
import BlogPostCard from '@/app/(pages)/blog/BlogPostCard';
import { Card, CardBody } from '@nextui-org/card';

import * as motion from 'motion/react-client';

const PostList = async ({ page }: { page: number }) => {
    try {
        const res = await fetchNocturneApi(`/blog/get_post_list?page=${ page }`);
        const posts = await res.json();
        return (
            // blog list
            <div className={ 'w-full' }>
                <ul className={ 'flex flex-col items-center w-full space-y-7' }>
                    { posts.map((post: JSON, key: number) => (
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
            // todo:  use Error.tsx
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: { e.toString() }
                </CardBody>
            </Card>);
    }
};
export default PostList;
