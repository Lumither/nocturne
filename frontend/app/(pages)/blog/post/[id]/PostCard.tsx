import React from 'react';
import { Card, CardBody } from '@nextui-org/card';
import Headers from '@/app/(pages)/blog/post/[id]/Headers';
import Footer from '@/app/(pages)/blog/post/[id]/Footer';
import MarkdownRenderer from '@/app/(pages)/public/MarkdownRenderer';

type Props = {
    post: any
}

async function PostCard(props: Props) {
    let post = props.post;
    return (
        <div>
            <Card className={ `max-w-full relative` }>

                {/* card headers */ }
                <Headers title={ post.title }
                         subtitle={ post.sub_title }
                         summary={ post.summary }
                         category={ post.category }
                         tags={ post.tags }
                         first_update={ post.first_update }
                         header_img={ post.meta.header_img }
                />

                {/* card body */ }
                <CardBody>
                    <div className={ 'p-5' }>
                        <MarkdownRenderer>
                            { post.content }
                        </MarkdownRenderer>
                    </div>
                </CardBody>


                {/* card footer */ }
                <Footer
                    adjacent_posts={ post.adj }
                    perm_link={ post.meta.link }
                    author={ post.meta.author }
                    author_link={ post.meta.author_link }
                    first_update={ post.first_update }
                    last_update={ post.last_update }
                    cc={ post.meta.cc }
                    post_id={ post.meta.id }
                    title={ post.title }
                />

            </Card>
        </div>
    );
}

export default PostCard;
