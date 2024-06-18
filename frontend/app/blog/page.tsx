import React from 'react';
import fetch_posts_list from '@/app/blog/fetch_posts_list';
import BlogPostCard from '@/app/blog/BlogPostCard';
import { Card, CardBody } from '@nextui-org/card';
import { MotionDiv } from '@/app/public/MotionDiv';

async function Blog() {

    try {
        const res = await fetch_posts_list();
        let posts = res['posts'];
        return (
            // blog list
            <ul className={ 'flex flex-col items-center w-full space-y-7' }>
                { posts.map((post: JSON, key: number) => (
                    <li key={ key } className={ 'w-full' }>
                        <MotionDiv
                            initial={ { y: 20, opacity: 0 } }
                            animate={ { y: 0, opacity: 1 } }
                            transition={ { ease: 'easeInOut', duration: 0.5, delay: key * .2 + .25 } }
                        >
                            <BlogPostCard post={ post } />
                        </MotionDiv>
                    </li>)
                ) }
            </ul>
        );
    } catch (e: any) {
        return (
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: Failed to load post list.
                </CardBody>
            </Card>);
    }

}

export default Blog;
