/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'oss.lumither.com'
            },
            {
                protocol: 'https',
                hostname: 'pic.re'
            }
        ]
    }
};

export default nextConfig;
