import axios from 'axios';

export default async function fetch_posts_list(): Promise<any> {
    const response = await axios(`${ process.env.BACKEND_URL }/get_post_list`);
    if (response.status !== 200) {
        throw new Error(response.statusText);
    }
    return response.data;
}
