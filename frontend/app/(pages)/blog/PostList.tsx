'use server';

import React from 'react';
import { fetchPostList } from '@/src/api/blog/post';
import AnimationPostList from '@/app/(pages)/blog/post/components/AnimationPostList';

const PostList = async ({ page }: { page: number }) => {
    try {
        const postListResponse = await fetchPostList(page);

        const posts = postListResponse.data.posts;

        return (
            // blog list
            <div className={ 'w-full' }>
                <AnimationPostList posts={ posts } />
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
