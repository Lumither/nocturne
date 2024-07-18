import axios from 'axios';

export default async function fetch_posts_list(): Promise<any> {
    const request_url = `${ process.env.BACKEND_URL }/get_post_list`;
    const response = await axios(request_url);
    if (response.status !== 200) {
        throw new Error(response.statusText);
    }
    return response.data;
}
