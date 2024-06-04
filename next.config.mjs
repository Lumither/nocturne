/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'oss.lumither.com'
            }
        ]
    }
};

export default nextConfig;
