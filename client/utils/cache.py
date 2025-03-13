import json
import os
from typing import TextIO

CACHE_DIR = ".cache"
ITEMS = {"register": "register.json"}


def overwrite_json_file(file: TextIO, json_data: dict):
    file.seek(0)
    file.write(json.dumps(json_data, indent=4, ensure_ascii=False))
    file.truncate()


def init_cache():
    if not os.path.exists(CACHE_DIR):
        os.makedirs(CACHE_DIR)
    for item in ITEMS.values():
        item_path = os.path.join(CACHE_DIR, item)
        if not os.path.exists(item_path):
            with open(item_path, 'w') as cache_file:
                match item:
                    case "register.json":
                        overwrite_json_file(cache_file, {})
                        break


def write_tags_cache(tags: list[str]):
    with open(os.path.join(CACHE_DIR, ITEMS["register"]), 'r+') as tags_cache_file:
        cache = json.load(tags_cache_file)
        cache["tags"] = tags
        overwrite_json_file(tags_cache_file, cache)


def write_category_cache(categories: list[str]):
    with open(os.path.join(CACHE_DIR, ITEMS["register"]), 'r+') as categories_cache_file:
        cache = json.load(categories_cache_file)
        cache["categories"] = categories
        overwrite_json_file(categories_cache_file, cache)


def get_tags_cache() -> list[str]:
    with open(os.path.join(CACHE_DIR, ITEMS["register"]), 'r') as tags_cache_file:
        return json.load(tags_cache_file)["tags"]


def get_categories_cache() -> list[str]:
    with open(os.path.join(CACHE_DIR, ITEMS["register"]), 'r') as categories_cache_file:
        return json.load(categories_cache_file)["categories"]
