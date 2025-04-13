import PostList from '@/app/(pages)/blog/PostList';
import Pagination from '@/app/(pages)/blog/Pagination';
import { fetchNocturneApi } from '@/app/(pages)/blog/api';

async function Blog({
    searchParams
}: {
    searchParams: Promise<{ [key: string]: string | string[] | undefined }>
}) {

    const parsePage = Number((await searchParams)?.page);
    const currPage = parsePage > 0 ? parsePage : 1;
    const ret = await fetchNocturneApi(`/blog/get_page_count`);
    const pageCount = (await ret.json() as any)['res'] as number;

    return (
        <div className={ 'w-full min-w-full space-y-4' }>
            <PostList page={ currPage } />
            <Pagination total={ pageCount } />
        </div>
    );
}

export default Blog;
