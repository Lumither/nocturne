import os
from abc import ABC, abstractmethod
from typing import Dict, Optional


class Template(ABC):
    @abstractmethod
    def init(self):
        pass


def create_all_files(relative_work_path: str, items: Dict[str, Optional[str]]):
    absolute_path = os.path.abspath(relative_work_path)
    for item in items:
        if item.endswith("/"):
            print(absolute_path + item)
    # todo: create files
