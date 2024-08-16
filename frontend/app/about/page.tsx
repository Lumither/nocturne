import { Card } from '@nextui-org/react';
import React from 'react';
import { CardBody } from '@nextui-org/card';
import Link from 'next/link';

const About = () => {
    return (
        <div className={ `flex flex-col w-full justify-center align-middle h-dvh` }>
            <div className={ 'md:relative md:h-[130px] mb-4 md:mb-0' }>
                <div className={ 'md:relative md:left-[20%] flex flex-col w-fit' }>
                    <p className={ 'text-pretty text-3xl' }>{ '讷，远道而来的' }<span
                        className={ 'break-keep underline decoration-sky-400 dark:decoration-sky-700 decoration-4 underline-offset-0' }>旅人</span>
                    </p>
                    <p className={ 'text-pretty md:text-2xl mt-1 md:ml-4' }>{ '愿意坐下来听个故事吗...' }</p>
                    <p className={ 'hidden md:block text-lg md:ml-8' }>{ '一个关于一位' }<span
                        className={ 'text-gray-700 dark:text-gray-400 text-sm transition-colors' }>{ '█' }</span>{ '追寻' }<span
                        className={ 'text-gray-700 dark:text-gray-400 text-sm transition-colors' }>{ '█' }</span>{ '，探索' }<span
                        className={ 'text-gray-700 dark:text-gray-400 text-sm transition-colors' }>{ '█' }</span>{ '的故事' }
                    </p>
                </div>
            </div>
            <div className={ `flex justify-center w-full h-[60%] md:h-[40%]` }>
                <div
                    className={ `flex flex-col md:flex-row w-full md:w-[85%] lg:w-[90%] xl:w-[85%] p-4 gap-x-7 gap-y-5` }>
                    <Card
                        isHoverable
                        className={ 'w-full h-[45%] md:h-full' }
                        as={ Link }
                        href={ '/about/me' }
                        isPressable
                    >
                        <CardBody>
                            <div className={ 'h-full flex flex-col justify-center' }>
                                <p className={ 'text-center text-3xl h-fit' }>关于我</p>
                            </div>
                        </CardBody>
                    </Card>
                    <Card
                        isHoverable
                        className={ 'w-full h-[45%] md:h-full' }
                        as={ Link }
                        href={ '/about/website' }
                        isPressable
                    >
                        <CardBody>
                            <div className={ 'h-full flex flex-col justify-center' }>
                                <p className={ 'text-center text-3xl h-fit' }>关于这个网站</p>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </div>
            <p className={ 'text-center' }>*目前还没想到如何设计这一页，先摸鱼会（笑）</p>
        </div>
    );
};

export default About;
