import React from 'react';
import Image from 'next/image';
import { Chip } from '@nextui-org/react';
import { MdCalendarMonth } from 'react-icons/md';

// type Props = {
//     post: any
// };

type Props = {
    title: string;
    subtitle: string;
    summary: string | undefined;
    category: string;
    tags: string[];
    first_update: string;
    header_img: string | undefined;
}


const Headers = (post: Props) => {

    return (
        <>
            { post.header_img && <div className={ `w-full overflow h-96 relative` }>
                <Image
                    alt={ 'cover picture' }
                    src={ post.header_img }
                    className={ `w-full object-cover rounded-b-none z-[5]` }
                    fill
                ></Image>
                <Image
                    alt={ 'cover picture' }
                    src={ post.header_img }
                    className={ `w-full relative scale-110 blur-3xl object-cover rounded-b-none` }
                    fill
                ></Image>
            </div> }

            <div
                className={ post.header_img ?
                    'mx-8 -mt-16 backdrop-blur transition bg-zinc-300/80 dark:bg-gray-600/25 rounded-2xl z-30'
                    : 'mx-6 mt-6'
                }>
                <div className={ post.header_img ? `p-4` : 'p-4 pt-2' }>
                    <Chip className={ `mb-2` } radius={ `sm` } color={ `secondary` }
                          variant={ `solid` }>{ post.category }</Chip>
                    <p className={ `text-4xl font-bold` }> { post.title }</p>
                    <p className={ `text-xl` }> { post.subtitle }</p>
                </div>

                <div className={ `mx-2 -mt-2 mb-2 flex flex-row flex-wrap justify-between w-full` }>
                    <div>
                        <ul className={ `flex flex-row flex-wrap` }>
                            {
                                post.tags.map((tag: string, key: any) => (
                                    <li key={ key } className={ `mx-1 mb-1` }>
                                        <Chip>{ tag }</Chip>
                                    </li>
                                ))
                            }
                        </ul>
                    </div>
                    <div className={ `flex flex-row place-content-end mb-1 mr-6 items-center mx-1` }>
                        <MdCalendarMonth />
                        <p className={ `text-zinc-500 dark:text-zinc-400 ml-1` }>
                            { (new Date(post.first_update)).toLocaleString('default', {
                                month: `long`,
                                day: `numeric`,
                                year: `numeric`
                            }) }
                        </p>
                    </div>
                </div>
            </div>

        </>
    );
};
export default Headers;
