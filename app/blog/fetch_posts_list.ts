export default async function fetch_posts_list(): Promise<any> {
    const response = await fetch(`http://localhost:${ process.env.BACKEND_PORT }/api/get/post_list`);
    if (!response.ok) {
        throw new Error(response.statusText);
    }
    return await response.json();
}


