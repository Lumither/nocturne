# This branch was deprecated.

In the initial plan of the blog, `raw.githubusercontent.com/<usr>/<branch>/<path_to_file>` (or any other git server)
was used as the cdn for the blog website, which also come with VCS without effort.

The idea is to build a separate repo that will be served as a database in ideal blog system structure, and all the
functionality related to CMS will be hosted by cloudflare worker.

```bash
master > tree                                                                 4s 19:32:15
.
├── idx.json
└── posts
    ├── another_article.md
    ├── hello_world.md
    └── idx.json

2 directories, 3 files
```

This is an example of [db repo](https://github.com/Lumither/blog-posts.git).

```bash

master > cat idx.json                                                         15:08:34
{
    "idx": [
        {
            "type": "dir",
            "name": "posts"
        }
    ]
}
master > cat ./posts/idx.json                                                 15:09:53
{
    "idx": [
        {
            "type": "md",
            "name": "another_article.md",
            "meta": {
                "title": "another_article!",
                "desc": "test",
                "tag": ["blog", "test", "dev"]
            }
        },
        {
            "type": "md",
            "name": "hello_world.md",
            "meta": {
                "title": "Hello World!",
                "desc": "test",
                "tag": ["blog", "test", "dev"]
            }
        }
    ]
}
```

These are examples of `idx.json`. They were designed to be placed at any layer of the directory, and used
to accelerate the client side browsing. The following is an easy [implementation](/app/blog/fetch_posts.ts) of using `idx.json` at `/posts` 
to get a list of blog posts.

```ts
export default async function fetch_posts(url: string): Promise<Result<JSON[], Error>> {
    const response = await fetch(`${ url }/posts/idx.json`);
    if (!response.ok) {
        return {
            ok: false,
            error: new Error(response.statusText)
        };
    }
    const res = await response.json();
    const posts = res['idx'].filter((posts: { type: string }) => posts.type === 'md');
    return {
        ok: true,
        value: posts
    };
}
```

The approach work impressively well, thanks for the globe services of GitHub lol, but eventually the plan was changed
due to ethical reasons and possibility of breaking GitHub's EULA. Plus, the functionality of the blog site might be constrained due to
Cloudflare worker's CPU time limitations, considering the existence of potential resource-intense work such as on-site searching and analytics.
