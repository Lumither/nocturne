import PostList from '@/app/(pages)/blog/PostList';
import Pagination from '@/app/(pages)/blog/Pagination';
import { fetchPostList } from '@/src/api/blog/post';
import ToolBar from '@/app/(pages)/blog/ToolBar';


async function Blog({
    searchParams
}: {
    searchParams: Promise<{ [_: string]: string | string[] | undefined }>
}) {

    const sp = await searchParams;

    const parsePage = Number(sp?.page);
    const currPage = parsePage > 0 ? parsePage : 1;

    const matchAll = Boolean(sp?.match_all);

    const ensureList = (input: string | string[] | undefined): string[] | undefined => {
        if (!input) {
            return undefined;
        }
        if (typeof input === 'string') {
            return [ input ];
        } else {
            return input;
        }
    };
    const tags = ensureList(sp?.tags);

    try {
        const postListResponse = await fetchPostList({
            page: currPage,
            match_all: matchAll,
            tag_filter: tags
        });

        const pagination = postListResponse.data.pagination;
        const posts = postListResponse.data.posts;

        return (
            <div className={ 'w-full min-w-full space-y-4' }>
                <ToolBar />
                <PostList posts={ posts } tags={ tags } />
                <Pagination total={ pagination.page_count } />
            </div>
        );
    } catch (e: any) {
        return (
            <div className={ `flex h-full min-h-dvh w-full justify-center items-center` }>
                <div className={ `flex-row` }>
                    <p className={ `text-xl font-bold` }>
                        { 'Unexpected Error [loading blog posts]:' }
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
