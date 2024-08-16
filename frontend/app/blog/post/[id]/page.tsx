import React from 'react';
import PostCard from '@/app/blog/post/[id]/PostCard';
import { MotionDiv } from '@/app/public/MotionDiv';
import ButtonGoBack from '@/app/blog/post/[id]/ButtonGoBack';

function BlogReader({ params }: { params: { id: string } }) {

    return (
        <MotionDiv
            className={ `max-w-full` }
            initial={ { y: 20, opacity: 0 } }
            animate={ { y: 0, opacity: 1 } }
            transition={ { ease: 'easeInOut', duration: 0.5 } }
        >
            <div className={ `max-w-full` }>
                <PostCard id={ params.id } />
            </div>
            <div className={ 'mt-3' }>
                <ButtonGoBack />
            </div>
        </MotionDiv>
    );
}

export default BlogReader;
