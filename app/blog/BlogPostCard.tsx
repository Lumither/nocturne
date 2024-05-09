import React from 'react';
import { Card, CardBody, CardFooter } from '@nextui-org/card';
import { Chip, Image } from '@nextui-org/react';
import Link from 'next/link';
import { MdCalendarMonth, MdNorthEast } from 'react-icons/md';

type Props = {
    post: JSON
}

function BlogPostCard({ post }: Props) {
    let title = (post as any)['title'];
    let sub_title = (post as any)['sub_title'];
    // let summary = (post as any)['summary'];
    let category = (post as any)['category'];
    let tags = (post as any)['tags'];
    // let last_update = new Date((post as any)['last_update']);
    let first_update = new Date((post as any)['first_update']);
    let post_id = (post as any)['post_id'];
    let pic_url = (post as any)['pic_url'];

    return (

        <div>
            <Card as={ Link } href={ `/blog/post/${ post_id }` }
                  className={ `mx-7` }
                  isPressable
                  isHoverable
            >
                { pic_url !== '' && <div className={ `w-full overflow-visible` }>
                    <Image
                        isZoomed
                        isBlurred
                        alt={ 'cover picture' }
                        src={ pic_url }
                        className={ `h-80 w-full object-cover` }
                        width={ `100%` }
                    ></Image>
                </div> }

                <CardBody>
                    <div className={ `flex flex-row justify-between` }>
                        <div className={ `mx-2 mt-2` }>
                            <Chip className={ `mb-2` } radius={ `sm` } color={ `secondary` }
                                  variant={ `solid` }>{ category }</Chip>
                            <p className={ `text-4xl font-bold` }> { title }</p>
                            <p className={ `text-xl` }> { sub_title }</p>
                        </div>
                        <MdNorthEast className={ `mt-4 mr-2` } size={ `40px` } />
                    </div>
                </CardBody>
                <CardFooter>
                    <div className={ `flex flex-row flex-wrap justify-between w-full` }>
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
                        <div className={ `flex flex-row place-content-end mb-1 items-center mx-1` }>
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
                </CardFooter>
            </Card>
        </div>
    );

}

export default BlogPostCard;
