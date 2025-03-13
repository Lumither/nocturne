import os

import frontmatter

from utils.cache import write_tags_cache, write_category_cache
from utils.lang import list_exclude

EXCLUDE = [".DS_Store"]
POSTS_DIR = "content/posts"


def build_post_idx():
    tags = set()
    categories = set()
    for year in list_exclude(os.listdir(POSTS_DIR), EXCLUDE):
        for post_dir in list_exclude(os.listdir(os.path.join(POSTS_DIR, year)), EXCLUDE):
            target_file = os.path.join(POSTS_DIR, year, post_dir, "index.md")
            with open(target_file, "r") as f:
                post = frontmatter.loads(f.read())
                try:
                    for tag in post.metadata["tags"]:
                        tags.add(tag)
                    categories.add(post.metadata["category"])
                except KeyError:
                    print(f"failed to parse {target_file}")
    write_tags_cache(list(tags))
    write_category_cache(list(categories))
