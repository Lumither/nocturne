import React from 'react';
import Link from 'next/link';
import { Card, CardBody, CardHeader } from '@nextui-org/card';
import { Divider } from '@nextui-org/react';

type Props = {
    post: JSON
}

function BlogPostCard({ post }: Props) {
    const name = (post as any)['name'];
    const { title, desc } = (post as any)['meta'] as { title: string, desc: string };
    return (

        <div>
            {/*todo: to be update*/ }
            {/*<Link href={ `/blog/${ name }` }>*/}
                <Card>
                    <CardHeader>
                        <p className={ 'font-bold' }>{ title }</p>
                    </CardHeader>
                    <Divider />
                    <CardBody>
                        { desc }
                    </CardBody>
                </Card>
            {/*</Link>*/}
        </div>
    );

}

export default BlogPostCard;
