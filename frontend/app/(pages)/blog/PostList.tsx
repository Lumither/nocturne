import React from 'react';
import { Post } from '@/src/api/blog/post';
import AnimationPostList, { HighlightSelector } from '@/app/(pages)/blog/post/components/AnimationPostList';

interface Props {
    posts: Post[],
    tags: string[] | undefined
}

const PostList = async ({ posts, tags }: Props) => {
    try {
        return (
            <div className={ 'w-full' }>
                <AnimationPostList posts={ posts } highlight={ {
                    tags: tags
                } } />
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
