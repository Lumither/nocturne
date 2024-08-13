const LOCAL_ENDPOINT = process.env.API_LOCAL_URL;
const REMOTE_ENDPOINT = process.env.API_REMOTE_URL;

const NOCTURNE_ENDPOINT = ((isServerSide = true) => {
    if (process.env.NODE_ENV === 'production') {
        if (isServerSide) {
            return LOCAL_ENDPOINT;
        } else {
            return REMOTE_ENDPOINT;
        }
    } else {
        return LOCAL_ENDPOINT;
    }
});

export async function fetchNocturneApi(path: string, isServerSideRequest = true) {
    return await fetch(`${ NOCTURNE_ENDPOINT(isServerSideRequest) }${ path }`, {
        cache: 'no-cache'
    });
}

