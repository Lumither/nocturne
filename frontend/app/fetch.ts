export async function fetchText(url: string, ttl: number = 600) {
    const res = await fetch(url, {
        next: {
            revalidate: ttl
        }
    });
    return await res.text();
}

export async function fetchJson(url: string, ttl: number = 600) {
    const res = await fetch(url, {
        next: {
            revalidate: ttl
        }
    });
    return await res.json();
}
