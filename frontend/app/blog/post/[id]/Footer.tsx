import React from 'react';
import MinifyPostCard from '@/app/blog/post/[id]/MinifyPostCard';
import Link from 'next/link';
import { FaCreativeCommons } from 'react-icons/fa6';

type Props = {
    post: any
}

const Footer = (props: Props) => {
    const { post } = props;

    const header_img = post['header_img'];
    const adjacent_posts = post['adj'];
    const prev_post = adjacent_posts['prev'];
    const next_post = adjacent_posts['next'];
    const link = post['meta']['link'] ?? `https://lumither.com/blog/post/${ post['meta']['id'] }`;
    const author = post['meta']['author'] ?? 'Tao';
    const author_link = post['meta']['author_link'] ?? '/about';
    const post_date = new Date(post['first_update']);
    const update_date = new Date(post['last_update']);
    const cc = post['meta']['cc'] ?? 'by-nc-sa';


    return (
        <div className={ `w-full` }>

            <div className={ 'flex transition bg-zinc-300/80 dark:bg-gray-600/25' }>
                <div className={ `p-8 w-full` }>
                    <div className={ 'flex flex-col space-y-4 relative' }>
                        <div className={ 'z-[2]' }>
                            <p className={ 'text-xl font-bold' }>{ post['meta']['title'] }</p>
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
                            <div>
                                <p className={ 'text-lg font-bold' }>Updated on</p>
                                <p>{ update_date.toLocaleString('default', {
                                    month: `long`,
                                    day: `numeric`,
                                    year: `numeric`
                                }) }</p>
                            </div>
                            <div>
                                <p className={ 'text-lg font-bold' }>Published under</p>
                                <Link href={ `https://creativecommons.org/licenses/${ cc }/4.0/` }>
                                    <p className={ 'underline decoration-indigo-500 dark:decoration-sky-500' }>{ `CC ${ cc.toUpperCase() } 4.0` }</p>
                                </Link>
                            </div>
                        </div>
                        <FaCreativeCommons size={ '80px' }
                                           className={ 'absolute inset-y-0 right-0 opacity-50 brightness-75  z-0' } />
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
