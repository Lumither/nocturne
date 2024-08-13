import PostList from '@/app/blog/PostList';
import Pagination from '@/app/blog/Pagination';
import { fetchNocturneApi } from '@/app/blog/api';

async function Blog({ searchParams }: {
    searchParams?: {
        page?: number
    };
}) {

    const parsePage = Number(searchParams?.page);
    const currPage = parsePage > 0 ? parsePage : 1;
    const ret = await fetchNocturneApi(`/get_page_count`);
    const pageCount = (await ret.json() as any)['res'] as number;

    return (
        <div className={ 'w-full min-w-full space-y-4' }>
            <PostList page={ currPage } />
            <Pagination total={ pageCount } />
        </div>
    );
}

export default Blog;
