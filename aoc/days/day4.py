import re

from aoc.days.day import Day


class Day4(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(4, test)

    def part1(self):
        return sum(1 for line in self.lines
                   if (bounds := list(map(int, re.split(r"[,-]", line))))
                   and (bounds[0] <= bounds[2] and bounds[1] >= bounds[3] or
                        bounds[0] >= bounds[2] and bounds[1] <= bounds[3]))

    def part2(self):
        return sum(1 for line in self.lines
                   if (bounds := list(map(int, re.split(r"[,-]", line))))
                   and not (bounds[2] > bounds[1] or bounds[0] > bounds[3]))
