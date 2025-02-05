import React from 'react';
import { fetchJson } from '@/app/fetch';
import { Card, CardBody } from '@nextui-org/react';
import Image from 'next/image';
import Link from 'next/link';

export default async function Friends() {

    const list = await fetchJson('https://raw.githubusercontent.com/Lumither/friends/master/friends.json');
    const listLength = list.length;
    const listMaxIdx = listLength - 1;
    const listIsOdd = listLength % 2 !== 0;

    return (
        <div>
            <div className={ `min-h-dvh pb-10` }>
                <div className={ 'p-7 pt-[10%] md:pt-[20%]' }>
                    <div className={ 'flex flex-col md:flex-row gap-x-3 justify-center' }>
                        <p className={ 'text-3xl text-center text-balance' }>海内存知己</p>
                        <p className={ 'text-3xl text-center text-balance' }>天涯若比邻</p>
                    </div>
                    <p className={ 'text-center text-balance text-zinc-500 dark:text-zinc-400' }>{ 'Though far apart, kin at heart' }</p>
                </div>
                <div className={ 'flex w-full h-full justify-center pt-3' }>
                    <ul className={ 'grid grid-cols-2 gap-x-6 gap-y-3 md:max-w-[90%] xl:max-w-[85%] w-full' }>
                        {
                            list.map((friend: any, key: number) => (
                                <li
                                    key={ key }
                                    className={ (key === listMaxIdx && listIsOdd) ? 'col-span-2 flex justify-center' : 'col-span-2 md:col-span-1' }
                                >
                                    <Card
                                        isHoverable
                                        isPressable
                                        shadow={ 'none' }
                                        as={ Link }
                                        href={ friend['url'] }
                                        className={ 'dark:bg-[#282830] w-full md:w-fit max-w-full shadow' }
                                    >
                                        <CardBody className={ 'overflow-visible pb-0.5' }>
                                            <div className={ 'flex flex-row' }>
                                                <div className={ 'flex p-2' }>
                                                    <div
                                                        className={ 'h-[80px] w-[80px] relative rounded-2xl' }>
                                                        <Image
                                                            alt="avatar"
                                                            className="object-cover rounded-2xl z-10"
                                                            fill
                                                            unoptimized
                                                            src={ friend['avatar'] }
                                                        />
                                                        <Image
                                                            alt="avatar"
                                                            className="relative object-cover rounded-2xl scale-110 blur-3xl"
                                                            fill
                                                            unoptimized
                                                            src={ friend['avatar'] }
                                                        />
                                                    </div>
                                                </div>
                                                <div className={ 'p-2 flex-col max-w-full' }>
                                                    <p className={ 'text-xl' }>{ friend['title'] }</p>
                                                    { (friend['name'] === undefined || friend['name'].isNull) ??
                                                        <p className={ 'text-zinc-600 dark:text-zinc-400' }>{ `@ ${ friend['name'] }` }</p> }
                                                    { (friend['desc'] === undefined || friend['desc'].isNull) ??
                                                        <p className={ 'text-sm text-pretty text-zinc-600 dark:text-zinc-300 mt-1' }>{ friend['desc'] }</p> }
                                                </div>
                                            </div>
                                        </CardBody>
                                        <div className={ 'h-2 relative w-full' }>
                                            <Image
                                                alt="theme bar"
                                                draggable={ false }
                                                className="object-cover object-center scale-110 blur-lg"
                                                fill
                                                unoptimized
                                                src={ friend['avatar'] }
                                            />
                                        </div>
                                    </Card>
                                </li>
                            ))
                        }
                    </ul>
                </div>
            </div>
            <p className={ 'text-center' }>想要成为好伙伴嘛，向<span
                className={ 'text-blue-800 dark:text-blue-500' }><Link
                href={ 'https://github.com/Lumither/friends/' }>这里</Link></span>发起PR吧
            </p>
        </div>
    );
}
