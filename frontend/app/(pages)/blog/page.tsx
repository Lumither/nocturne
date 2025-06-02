import PostList from '@/app/(pages)/blog/PostList';
import Pagination from '@/app/(pages)/blog/Pagination';
import { fetchPostListPagination } from '@/src/api/blog/post';


async function Blog({
    searchParams
}: {
    searchParams: Promise<{ [_: string]: string | string[] | undefined }>
}) {

    const parsePage = Number((await searchParams)?.page);
    const currPage = parsePage > 0 ? parsePage : 1;


    try {
        const pagination = await fetchPostListPagination();
        const pageCount = pagination.data.page_count;

        return (
            <div className={ 'w-full min-w-full space-y-4' }>
                <PostList page={ currPage } />
                <Pagination total={ pageCount } />
            </div>
        );
    } catch (e: any) {
        return (
            <div className={ `flex h-full min-h-dvh w-full justify-center items-center` }>
                <div className={ `flex-row` }>
                    <p className={ `text-xl font-bold` }>
                        { 'Unexpected Error [loading pagination]:' }
                    </p>
                    <p className={ `font-bold` }>
                        { e.message }
                    </p>
                </div>
            </div>
        );
    }


}

export default Blog;
