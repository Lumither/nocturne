import React, { cache } from 'react';
import PostCard from '@/app/(pages)/blog/post/[id]/PostCard';
import ButtonGoBack from '@/app/(pages)/blog/post/[id]/ButtonGoBack';
import * as motion from 'motion/react-client';
import { fetchNocturneApi } from '@/app/(pages)/blog/api';
import { Metadata } from 'next';
import { SITE_CONFIG } from '@/src/constants';

const fetchPost = cache(async (id: string) => {
    const res = await fetchNocturneApi(`/blog/get_post/${ id }`);
    // throw Error('Not Found');
    return await res.json();
});

export async function generateMetadata(
    { params }: { params: Promise<{ id: string }> })
    : Promise<Metadata> {
    try {
        const title = (await fetchPost((await params).id)).title as string;
        return {
            title: `${ title } - ${ SITE_CONFIG.name }`
        };
    } catch (e) {
        return {
            title: 'Unexpected Error'
        };
    }
}

async function BlogReader({
    params
}: {
    params: Promise<{ id: string }>
}) {
    try {
        let post = await fetchPost((await params).id);

        return (
            <motion.div
                className={ `max-w-full` }
                initial={ { y: 20, opacity: 0 } }
                animate={ { y: 0, opacity: 1 } }
                transition={ { ease: 'easeInOut', duration: 0.5 } }
            >
                <div className={ `max-w-full` }>
                    <PostCard post={ post } />
                </div>
                <div className={ 'mt-3' }>
                    <ButtonGoBack />
                </div>
            </motion.div>
        );
    } catch (e: any) {
        return (
            <div className={ `flex h-full w-full justify-center items-center` }>
                <div className={ `flex-row` }>
                    <p className={ `text-xl font-bold` }>
                        { 'Unexpected Error:' }
                    </p>
                    <p className={ `font-bold` }>
                        { e.message }
                    </p>
                </div>
            </div>
        );
    }

}

export default BlogReader;
