import React, { useEffect, useState } from 'react';
import { Activity } from '@/src/HomePageActivities/activity';
import { getGhActivities } from '@/src/HomePageActivities/ghActivity';
import * as motion from 'motion/react-client';

const Activities = () => {

    const [ activities, setActivities ] = useState<Activity[] | null>(null);

    useEffect(() => {
        getGhActivities().then(data => setActivities(data));
    }, []);

    if (!activities) {
        return (
            <div>
                <p>loading</p>
            </div>
        );
    } else {
        return (
            <div>
                <ul className={ 'flex flex-col gap-y-3 items-end' }>
                    { activities.map((activity, key) => (
                        <motion.div
                            key={ key }
                            initial={ { opacity: 0, y: 20 } }
                            animate={ { opacity: 1, y: 0 } }
                            transition={ { ease: 'easeInOut', duration: 0.2 } }
                        >
                            <li className={ 'backdrop-blur bg-white/75 dark:bg-black/45 rounded-xl w-fit' }>
                                { activity.toElement() }
                            </li>
                        </motion.div>
                    )) }
                </ul>
            </div>
        );
    }

};

export default Activities;