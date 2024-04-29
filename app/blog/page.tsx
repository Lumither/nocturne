import React from 'react';
import fetch_posts_list from '@/app/blog/fetch_posts_list';
import BlogPostCard from '@/app/blog/BlogPostCard';

async function Blog() {

    const res = await fetch_posts_list();
    if (!res.ok) {
        console.log(res.error);
        return;
    }
    const posts = res.value;

    console.log(posts);

    return (

        // blog list
        <ul className={ 'flex flex-col items-center w-full max-w-[1024px]' }>
            { posts.map((post, key) => (
                <li key={ key } className={ 'my-3.5 mx-7 w-[80%]' }>
                    <BlogPostCard post={ post } />
                </li>)
            ) }
        </ul>
    );
}

export default Blog;
