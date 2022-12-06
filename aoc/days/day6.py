from aoc.days.day import Day


class Day6(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(6, test)

    def part1(self):
        return self.__fin_n_unique(4)

    def part2(self):
        return self.__fin_n_unique(14)

    def __fin_n_unique(self, n):
        inp = self.lines[0]
        ans = n - 1
        while len(set(inp[ans - n + 1:ans + 1])) < n:
            ans += 1
        return ans + 1
