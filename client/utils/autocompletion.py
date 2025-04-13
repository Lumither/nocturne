from collections.abc import Callable
from typing import List, Iterable

import prompt_toolkit
from prompt_toolkit.completion import Completer, CompleteEvent, Completion
from prompt_toolkit.document import Document
from pypinyin import lazy_pinyin, Style


class AutoCompleter(Completer):
    def __init__(self, candidates: List[str], matching_algorithm: Callable[[str, str], bool]):
        self.candidates = candidates
        self.matching_algorithm = matching_algorithm

    def get_completions(self, document: Document, complete_event: CompleteEvent) -> Iterable[Completion]:
        curr_word = document.text_before_cursor
        for c in self.candidates:
            if self.matching_algorithm(curr_word, c):
                yield Completion(c, start_position=-len(curr_word))


def exact_matcher(word: str, candidate: str, case_sensitive: bool = False) -> bool:
    if case_sensitive:
        return word == candidate
    else:
        return word.lower() == candidate.lower()


def fuzzy_matcher(word: str, candidate: str) -> bool:
    return word.lower() in candidate.lower()


def fuzzy_pinyin_matcher(word: str, candidate: str) -> bool:
    try:
        word_matcher = ''.join(lazy_pinyin(word, style=Style.NORMAL))
        candidate_matcher = ''.join(lazy_pinyin(candidate, style=Style.NORMAL))
        return candidate_matcher.startswith(word_matcher)

    except e:
        return fuzzy_matcher(word, candidate)


def input_autocompletion(prompt: str, candidates: List[str],
                         matching_algorithm: Callable[[str, str], bool] = exact_matcher) -> str:
    completer = AutoCompleter(candidates, matching_algorithm)
    return prompt_toolkit.prompt(prompt, completer=completer, complete_in_thread=True)
