import React from 'react';
import Generate from '@/app/blog/post/[id]/generate';

function BlogReader({ params }: { params: { id: string } }) {

    return (
        <div className={ `mx-7 my-7 w-full` }>
            <Generate id={ params.id } />
        </div>
    );
}

export default BlogReader;
