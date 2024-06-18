import React from 'react';
import Generate from '@/app/blog/post/[id]/generate';
import { MotionDiv } from '@/app/public/MotionDiv';

function BlogReader({ params }: { params: { id: string } }) {

    return (
        <MotionDiv
            initial={ { y: 20, opacity: 0 } }
            animate={ { y: 0, opacity: 1 } }
            transition={ { ease: 'easeInOut', duration: 0.5 } }
        >
            <div className={ `w-full` }>
                <Generate id={ params.id } />
            </div>
        </MotionDiv>
    );
}

export default BlogReader;
