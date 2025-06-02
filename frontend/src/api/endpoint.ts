const LOCAL_ENDPOINT = process.env.API_LOCAL_URL;
const REMOTE_ENDPOINT = process.env.API_REMOTE_URL;
const FALLBACK_ENDPOINT = 'http://localhost:3001';

const NOCTURNE_ENDPOINT = ((isServerSide = true) => {
    const get_endpoint = () => {
        if (process.env.NODE_ENV === 'production') {
            if (isServerSide) {
                return LOCAL_ENDPOINT;
            } else {
                return REMOTE_ENDPOINT;
            }
        } else {
            return LOCAL_ENDPOINT;
        }
    };

    return get_endpoint() ?? FALLBACK_ENDPOINT;
});

export async function fetchNocturneJson<T>(
    path: string,
    isServerSideRequest = true,
    method: string = 'GET'
): Promise<T> {
    const url = `${ NOCTURNE_ENDPOINT(isServerSideRequest) }${ path }`;

    const future = fetch(url, {
        cache: 'no-cache',
        method: method
    });

    const res = (await future);

    if (!res.ok) {
        throw new Error(`Failed to fetch "${ url }" [${ res.status }]: ${ res.statusText }`);
    }

    return (await res.json()) as T;
}
