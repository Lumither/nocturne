import React from 'react';
import { Card, CardFooter, CardHeader } from '@nextui-org/card';
import Image from 'next/image';
import Link from 'next/link';

type Props = {
    post: any
    desc: string
}

const MinifyPostCard = (props: Props) => {
    const { post, desc } = props;
    const img_url = post['header_img'] as string;
    return (
        <div className={ 'w-full transition hover:brightness-75' }>
            <Card isFooterBlurred
                  className="w-full h-[300px] shadow-none border-0 rounded-none overflow-hidden"
                  as={ Link }
                  isPressable
                  href={ `/blog/post/${ post['id'] }` }
            >
                <CardHeader className="absolute z-10 top-1 flex-col items-start">
                    <p className={ `text-tiny uppercase font-bold ${ img_url ? 'text-white/60' : 'text-black/60 dark:text-white/60' }` }>{ desc }</p>
                </CardHeader>
                { img_url &&
                    <div>
                        <Image
                            className={ 'object-cover object-center ' }
                            src={ img_url }
                            alt={ post.title }
                            fill
                        />
                        <CardFooter
                            className="absolute transition-colors bg-zinc-300/80 dark:bg-gray-600/25 dark:backdrop-brightness-75 bottom-0  justify-between rounded-none">
                            <div>
                                <p className="text-2xl font-bold">{ post['title'] }</p>
                                <p className="">{ post['sub_title'] }</p>
                            </div>
                        </CardFooter>
                    </div>
                }
                { !img_url &&
                    <div className={ 'h-full grid p-6 place-content-center' }>
                        <div className={ 'flex flex-row  items-center text-center max-h-fit' }>
                            <p className={ 'text-3xl font-bold' }>{ post['title'] }</p>
                            <div className={ 'h-full w-1 mx-2 bg-zinc-500 dark:bg-zinc-400' }></div>
                            <p className={ 'text-xl' }>{ post['sub_title'] }</p>
                        </div>
                    </div>
                }
            </Card>

        </div>
    );
};
export default MinifyPostCard;
