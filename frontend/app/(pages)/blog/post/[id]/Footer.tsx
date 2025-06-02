import React from 'react';
import MinifyPostCard from '@/app/(pages)/blog/post/[id]/MinifyPostCard';
import Link from 'next/link';
import { FaCreativeCommons } from 'react-icons/fa6';

interface AdjacentPosts {
    prev: any;
    next: any;
}

type Props = {
    adjacent_posts: AdjacentPosts
    perm_link: string;
    author: string;
    author_link: string;
    first_update: string;
    last_update: string | null;
    cc: string | undefined;
    title: string;
}

const Footer = (props: Props) => {
    const { adjacent_posts, perm_link, author_link, author, first_update, last_update, title } = props;

    const prev_post = adjacent_posts.prev;
    const next_post = adjacent_posts.next;
    const link = perm_link;
    const post_date = new Date(first_update);
    const update_date: Date | undefined = last_update ? new Date(last_update) : undefined;
    const cc = props.cc ?? 'by-nc-sa';


    return (
        <div className={ `w-full` }>

            <div className={ 'flex transition bg-zinc-300/80 dark:bg-gray-600/25' }>
                <div className={ `p-8 w-full overflow-hidden` }>
                    <div className={ 'relative' }>
                        <FaCreativeCommons
                            size={ '160px' }
                            className={ 'absolute right-0 top-0 -mt-[4.8rem] -mr-16 opacity-30 brightness-50  ' }
                        />
                    </div>

                    <div className={ 'flex flex-col space-y-4 relative' }>
                        <div className={ 'z-[2]' }>
                            <p className={ 'text-xl font-bold' }>{ title }</p>
                            <Link href={ link }>
                                <p className={ 'text-sm underline decoration-indigo-500 dark:decoration-sky-500' }>{ link }</p>
                            </Link>
                        </div>
                        <div className={ 'z-[2] flex flex-row flex-wrap gap-x-7 gap-y-2' }>
                            <div>
                                <p className={ 'text-lg font-bold' }>Author</p>
                                <Link href={ author_link }>
                                    <p className={ 'underline decoration-indigo-500 dark:decoration-sky-500' }>{ author }</p>
                                </Link>
                            </div>
                            <div>
                                <p className={ 'text-lg font-bold' }>Posted on</p>
                                <p>{ post_date.toLocaleString('default', {
                                    month: `long`,
                                    day: `numeric`,
                                    year: `numeric`
                                }) }</p>
                            </div>
                            { update_date &&
                                <div>
                                    <p className={ 'text-lg font-bold' }>Updated on</p>
                                    <p>{ update_date.toLocaleString('default', {
                                        month: `long`,
                                        day: `numeric`,
                                        year: `numeric`
                                    }) }</p>
                                </div>
                            }
                            <div>
                                <p className={ 'text-lg font-bold' }>Published under</p>
                                <Link href={ `https://creativecommons.org/licenses/${ cc }/4.0/` }>
                                    <p className={ 'underline decoration-indigo-500 dark:decoration-sky-500' }>{ `CC ${ cc.toUpperCase() } 4.0` }</p>
                                </Link>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div className={ `flex justify-center` }>
                <div className={ `w-full shadow-2xl ` }>
                    <div className={ `flex flex-col md:flex-row w-full` }>
                        {
                            next_post &&
                            <MinifyPostCard post={ next_post } desc={ 'prev' }></MinifyPostCard>
                        }
                        {
                            prev_post &&
                            <MinifyPostCard post={ prev_post } desc={ 'next' }></MinifyPostCard>
                        }
                    </div>
                </div>
            </div>
        </div>
    );
};
export default Footer;
