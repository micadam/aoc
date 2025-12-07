import functools
from typing import Union

from aoc.day import Day


@functools.cache
def num_splits(
    grid: tuple[str, ...], y: int, x: int, quantum: bool = False
) -> Union[int, set[tuple[int, int]]]:
    if y == len(grid) - 1:
        return 1 if quantum else set()
    if grid[y][x] in ".S":
        return num_splits(grid, y + 1, x, quantum)
    if grid[y][x] == "^":
        if quantum:
            left = num_splits(grid, y, x - 1, True)
            right = num_splits(grid, y, x + 1, True)
            if not isinstance(left, int) or not isinstance(right, int):
                raise ValueError(f"Expected int, not int: {left}, {right}")
            return left + right
        else:
            left = num_splits(grid, y, x - 1, False)
            right = num_splits(grid, y, x + 1, False)
            if not isinstance(left, set) or not isinstance(right, set):
                raise ValueError(f"Expected set, not set: {left}, {right}")
            return set([(y, x)]).union(left).union(right)
    raise Exception(f"Invalid char: {grid[y][x]}")


class Day7(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(7, test)

    def part1(self):
        start_idx = next(i for i, c in enumerate(self.lines[0]) if c == "S")
        ans = num_splits(tuple(self.lines), 0, start_idx)
        if not isinstance(ans, set):
            raise ValueError(f"Expected set, not set: {ans}")
        return len(ans)

    def part2(self):
        start_idx = next(i for i, c in enumerate(self.lines[0]) if c == "S")
        ans = num_splits(tuple(self.lines), 0, start_idx, True)
        if not isinstance(ans, int):
            raise ValueError(f"Expected int, not int: {ans}")
        return ans
