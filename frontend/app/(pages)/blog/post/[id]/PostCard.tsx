import React from 'react';
import { Card, CardBody } from '@nextui-org/card';
import Headers from '@/app/(pages)/blog/post/[id]/Headers';
import Footer from '@/app/(pages)/blog/post/[id]/Footer';
import MarkdownRenderer from '@/app/(pages)/public/MarkdownRenderer';
import { FetchPostResponseData } from '@/src/api/blog/post';

type Props = {
    post: FetchPostResponseData
}

async function PostCard(props: Props) {
    const post = props.post.post;
    const metadata: any | null = post.metadata;
    const adj = props.post.adjacent;

    return (
        <div>
            <Card className={ `max-w-full relative` }>

                {/* card headers */ }
                <Headers title={ post.title }
                         subtitle={ post.subtitle }
                         category={ post.category }
                         tags={ post.tags }
                         first_update={ post.date_created }
                         header_img={ metadata?.header_img }
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
                    adjacent_posts={ adj }
                    perm_link={ metadata?.perm_link ?? `https://lumither.com/blog/post/${ post.id }` }
                    author={ metadata?.author ?? 'Tao' }
                    author_link={ metadata?.author_link ?? '/about/me' }
                    first_update={ post.date_created }
                    last_update={ post.date_updated }
                    cc={ metadata?.cc }
                    title={ post.title }
                />

            </Card>
        </div>
    );
}

export default PostCard;
