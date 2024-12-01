import React from 'react';
import { Card, CardBody } from '@nextui-org/card';
import Headers from '@/app/blog/post/[id]/Headers';
import Footer from '@/app/blog/post/[id]/Footer';
import { fetchNocturneApi } from '@/app/blog/api';
import MarkdownRenderer from '@/app/public/MarkdownRenderer';

type Props = {
    id: string
}

async function PostCard(props: Props) {


    try {
        const res = await fetchNocturneApi(`/blog/get_post/${ props.id }`);
        const ret: any = await res.json();
        return (
            <div>
                <Card className={ `max-w-full relative` }>

                    {/* card headers */ }
                    <Headers post={ ret } />

                    {/* card body */ }
                    <CardBody>
                        <div className={ 'p-5' }>
                            <MarkdownRenderer>
                                { ret['content'] }
                            </MarkdownRenderer>
                        </div>
                    </CardBody>


                    {/* card footer */ }
                    <Footer post={ ret } />

                </Card>
            </div>
        );
    } catch (e: any) {
        return <div>
            <Card className={ `w-full` }>
                <CardBody>
                    Fatal: { e.toString() }
                </CardBody>
            </Card>
        </div>;
    }

}

export default PostCard;
