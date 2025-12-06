from functools import reduce

from aoc.day import Day


def calc(operation, numbers):
    s = {"+": 0, "*": 1}[operation]
    op = {"+": int.__add__, "*": int.__mul__}[operation]
    return reduce(op, numbers, s)


class Day6(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(6, test)

    def part1(self):
        lines_split = [line.split() for line in self.lines]
        number_lines, operations = lines_split[:-1], lines_split[-1]
        ans = 0
        number_lines = zip(*number_lines)
        for operation, line in zip(operations, number_lines):
            numbers = [int(num) for num in line]
            ans += calc(operation, numbers)
        return ans

    def part2(self):
        number_lines, op_line = self.lines[:-1], self.lines[-1]
        operations = op_line.split()
        naive_split = [line.split() for line in number_lines]
        numbers = zip(*naive_split)
        widths = [
            reduce(lambda acc, new: max(acc, len(new)), group, 0) for group in numbers
        ]

        ans = 0
        skip = 0
        for operation, width in zip(operations, widths):
            # the ljust is just for the ends of lines where the length may dffer
            numbers = [line[skip : skip + width].ljust(width) for line in number_lines]
            numbers = list(zip(*numbers))
            numbers = [int("".join(num)) for num in numbers]
            ans += calc(operation, numbers)
            skip += width + 1
        return ans
