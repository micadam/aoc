from math import floor, log
from typing import List, Literal

from aoc.days.day import Day

VALS = {
    '2': 2,
    '1': 1,
    '0': 0,
    '-': -1,
    '=': -2,
}

CHARS = {v: k for k, v in VALS.items()}

LOG5 = log(5)


def log5(n: float):
    return log(n) / LOG5


class Day25(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(25, test)

    def part1(self):
        return self.to_snafu(sum(map(self.to_int, self.lines)))

    def part2(self):
        print("Merry Christmas!")

    def to_int(self, snafu: str) -> int:
        ans = 0
        for c in snafu:
            ans *= 5
            ans += VALS[c]
        return ans

    def to_snafu(self, decimal: int) -> str:
        digits = []
        pow = 5 ** floor(log5(decimal))
        while pow:
            div = decimal // pow
            decimal %= pow
            pow //= 5
            digits.append(div)
            idx = len(digits) - 1
            while idx > 0 and digits[idx] > 2:
                digits[idx - 1] += 1
                digits[idx] -= 5
                idx -= 1
            if digits[0] > 2:
                digits[0] -= 5
                digits.insert(0, 1)
        assert all(c in CHARS for c in digits)
        return "".join(map(CHARS.get, digits))  # type: ignore
