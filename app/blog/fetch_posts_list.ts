import axios from 'axios';

export default async function fetch_posts_list(): Promise<any> {
    const response = await axios(`http://localhost:${ process.env.BACKEND_PORT }/api/get/post_list`);
    if (response.status !== 200) {
        throw new Error(response.statusText);
    }
    return response.data;
}
