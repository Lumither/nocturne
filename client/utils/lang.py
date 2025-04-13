from utils.ansi import Ansi


def list_exclude(source: list, exclude: list):
    return filter(lambda item: item not in exclude, source)


def eprint(msg: str):
    print(Ansi.blend([Ansi.FAIL], "error: ") + msg)


def wprint(msg: str):
    print(Ansi.blend([Ansi.WARNING], "warn: ") + msg)