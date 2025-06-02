import { fetchNocturneJson } from '@/src/api/endpoint';

const DEFAULT_PAGE_SIZE = 6;

export interface PostListResponse {
    data: {
        posts: {
            id: string,
            identifier: string,
            title: string,
            subtitle: string,
            tags: string[]
            category: string,
            date_created: Date,
            date_updated: Date | null,
            header_img: string | null
        }[],
        pagination: {
            page: number,
            page_count: number,
            page_size: number
        }
    },
    status: string
}


export async function fetchPostList(page: number, page_size: number = DEFAULT_PAGE_SIZE): Promise<PostListResponse> {
    const url = `/blog/posts?page=${ page }&page_size=${ page_size }`;
    return await fetchNocturneJson(url);
}


export interface PostListPaginationResponseData {
    page_count: number,
    post_count: number,
}

export interface PostListPaginationResponse {
    data: PostListPaginationResponseData;
    status: string;
}

export async function fetchPostListPagination(): Promise<PostListPaginationResponse> {
    const url = `/blog/posts/pagination`;
    return await fetchNocturneJson(url);
}


export interface FetchPostResponseData {
    post: {
        id: string,
        identifier: string,
        title: string,
        subtitle: string,
        status: string,
        date_created: string,
        date_updated: string | null,
        category: string,
        tags: string[],
        content: string,
        metadata: any | null,
    },
    adjacent: {
        prev: {
            id: string,
            identifier: string,
            title: string,
            subtitle: string,
            date_created: string,
            category: string,
            tags: string[],
            header_img: string | null,
        } | null,
        next: {
            id: string,
            identifier: string,
            title: string,
            subtitle: string,
            date_created: string,
            category: string,
            tags: string[],
            header_img: string | null,
        } | null,
    }
}

export interface FetchPostResponse {
    data: FetchPostResponseData;
    status: string;
}

export async function fetchPost(id: string): Promise<FetchPostResponse> {
    const url = `/blog/posts/${ id }`;
    return await fetchNocturneJson(url);
}