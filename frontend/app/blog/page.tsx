import React from 'react';
import BlogPostCard from '@/app/blog/BlogPostCard';
import { Card, CardBody } from '@nextui-org/card';
import { MotionDiv } from '@/app/public/MotionDiv';
import { fetchNocturneApi } from '@/app/blog/api';

async function Blog() {

    const { code, ret } = await fetchNocturneApi('/get_post_list');
    if (code == 200) {
        let posts = ret['posts'];
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
    } else {
        return (
            // todo:  use Error.tsx
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: { ret }
                </CardBody>
            </Card>);
    }
}

export default Blog;
