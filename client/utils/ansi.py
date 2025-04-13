class Ansi:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    GREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    END = '\033[0m'

    def disable(self) -> None:
        self.HEADER = ''
        self.BLUE = ''
        self.GREEN = ''
        self.WARNING = ''
        self.FAIL = ''
        self.BOLD = ''
        self.UNDERLINE = ''
        self.END = ''

    @staticmethod
    def blend(color: list[str], text: str) -> str:
        return f"{"".join(color)}{text}{Ansi.END}"

    @staticmethod
    def bold(text: str) -> str:
        return f"{Ansi.BOLD}{text}{Ansi.END}"

    @staticmethod
    def underline(text: str) -> str:
        return f"{Ansi.UNDERLINE}{text}{Ansi.END}"
