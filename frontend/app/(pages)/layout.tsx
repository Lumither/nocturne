import React from 'react';
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';

import * as motion from 'motion/react-client';

function Layout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <div>
            <motion.div
                initial={ { opacity: 0 } }
                animate={ { opacity: 1 } }
                transition={ { ease: 'easeInOut', duration: 0.3, delay: 0.5 } }
                className={ `justify-center flex flex-row w-full max-w-full` }>
                <Navbar />
                <div className={ `flex flex-col flex-1 max-w-[1024px] min-w-0` }>
                    { children }
                    <Footer />
                </div>
            </motion.div>
        </div>
    )
        ;
}

export default Layout;
