import React from 'react';
import Generate from '@/app/blog/post/[id]/generate';

function BlogReader({ params }: { params: { id: string } }) {

    return (
        <div className={ `w-full` }>
            <Generate id={ params.id } />
        </div>
    );
}

export default BlogReader;
