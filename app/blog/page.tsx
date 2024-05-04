import React from 'react';
import fetch_posts_list from '@/app/blog/fetch_posts_list';
import BlogPostCard from '@/app/blog/BlogPostCard';

async function Blog() {

    try {
        const res = await fetch_posts_list();
        let posts = res['posts'];
        return (
            // blog list
            <ul className={ 'flex flex-col items-center w-full max-w-[1024px]' }>
                { posts.map((post: JSON, key: React.Key | null | undefined) => (
                    <li key={ key } className={ 'my-7 w-full' }>
                        <BlogPostCard post={ post } />
                    </li>)
                ) }
            </ul>
        );
    } catch (e: any) {
        return (
            <>
                { e.toString() }
            </>
        );
    }

}

export default Blog;
