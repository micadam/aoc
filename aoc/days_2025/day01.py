from functools import reduce

from aoc.day import Day


def consume(acc: int, score, next: str, only_final_click=True) -> tuple[int, int]:
    sign = 1 if next[0] == "R" else -1 if next[0] == "L" else None
    if not sign:
        raise ValueError("Unknown sigh")
    new_acc_raw = acc + sign * int(next[1:])
    new_acc = new_acc_raw % 100

    if only_final_click:
        score += new_acc == 0
    else:
        step = 1 if new_acc_raw > acc else -1
        for pos_0 in range(acc + step, new_acc_raw + step, step):
            if pos_0 % 100 == 0:
                score += 1

    return new_acc, score


class Day1(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(1, test)

    def part1(self):
        return reduce(
            lambda acc, next: consume(acc[0], acc[1], next), self.lines, (50, 0)
        )[1]

    def part2(self):
        return reduce(
            lambda acc, next: consume(acc[0], acc[1], next, only_final_click=False),
            self.lines,
            (50, 0),
        )[1]
