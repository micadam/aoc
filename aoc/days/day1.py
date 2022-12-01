import heapq

from aoc.days.day import Day


class Day1(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(1, test)

    def part1(self):
        ma = -1
        su = 0
        for cal in self.lines:
            if not cal:
                ma = max(ma, su)
                su = 0
                continue
            su += int(cal)
        ma = max(ma, su)
        return ma

    def part2(self):
        su = 0
        sus = []
        for cal in self.lines:
            if not cal:
                sus.append(su)
                su = 0
                continue
            su += int(cal)
        sus.append(su)
        return sum(heapq.nlargest(3, sus))
