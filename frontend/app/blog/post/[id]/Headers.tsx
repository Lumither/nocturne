import React from 'react';
import Image from 'next/image';
import { Chip } from '@nextui-org/react';
import { MdCalendarMonth } from 'react-icons/md';

type Props = {
    post: any
};


const Headers = (props: Props) => {
    let post = props.post;
    let title = post['title'];
    let sub_title = post['sub_title'];
    // let summary = (post as any)['summary'];
    let category = post['category'];
    let tags = post['tags'];
    // let last_update = new Date((post as any)['last_update']);
    let first_update = new Date(post['first_update']);
    // let post_id = (post as any)['post_id'];
    let header_img = post['header_img'];

    return (
        <>
            { header_img !== '' && <div className={ `w-full overflow h-96 relative` }>
                <Image
                    alt={ 'cover picture' }
                    src={ header_img }
                    className={ `w-full object-cover rounded-b-none z-[5]` }
                    fill
                ></Image>
                <Image
                    alt={ 'cover picture' }
                    src={ header_img }
                    className={ `w-full relative scale-110 blur-3xl object-cover rounded-b-none` }
                    fill
                ></Image>
            </div> }

            <div
                className={ header_img !== '' ?
                    'mx-8 -mt-16 backdrop-blur transition bg-zinc-300/80 dark:bg-gray-600/25 rounded-2xl z-30'
                    : 'mx-6 mt-6'
                }>
                <div className={ header_img !== '' ? `p-4` : 'p-4 pt-2' }>
                    <Chip className={ `mb-2` } radius={ `sm` } color={ `secondary` }
                          variant={ `solid` }>{ category }</Chip>
                    <p className={ `text-4xl font-bold` }> { title }</p>
                    <p className={ `text-xl` }> { sub_title }</p>
                </div>

                <div className={ `mx-2 -mt-2 mb-2 flex flex-row flex-wrap justify-between w-full` }>
                    <div>
                        <ul className={ `flex flex-row flex-wrap` }>
                            {
                                tags.map((tag: string, key: any) => (
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
                            { first_update.toLocaleString('default', {
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
