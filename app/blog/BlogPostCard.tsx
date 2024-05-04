import React from 'react';
import { Card, CardBody, CardHeader } from '@nextui-org/card';
import { Divider } from '@nextui-org/react';
import Link from 'next/link';

type Props = {
    post: JSON
}

function BlogPostCard({ post }: Props) {
    let title = (post as any)['title'];
    let summary = (post as any)['summary'];
    let last_update = (post as any)['last_update'];
    let post_id = (post as any)['post_id'];

    return (

        <div>
            <Card as={ Link } href={ `/blog/post/${ post_id }` } className={ `mx-7` }>
                <CardHeader>
                    <p className={ 'font-bold' }>{ title }</p>
                </CardHeader>
                <Divider />
                <CardBody>
                    { summary }
                </CardBody>
                <Divider />
                <CardBody>
                    { last_update }
                </CardBody>
            </Card>
        </div>
    );

}

export default BlogPostCard;
