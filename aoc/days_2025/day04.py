from aoc.day import Day
from aoc.util import get_grid_neighbours


def remove_rolls(
    grid: list[str], removed_already: set[tuple[int, int]]
) -> set[tuple[int, int]]:
    Y = len(grid)
    X = len(grid[0])
    ans = set()
    for y in range(Y):
        for x in range(X):
            if (
                (y, x) not in removed_already
                and grid[y][x] == "@"
                and sum(
                    grid[ny][nx] == "@" and (ny, nx) not in removed_already
                    for ny, nx in get_grid_neighbours(Y, X, y, x, include_diag=True)
                )
                < 4
            ):
                ans.add((y, x))
    return ans


class Day4(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(4, test)

    def part1(self):
        return len(remove_rolls(self.lines, set()))

    def part2(self):
        done = False
        removed = set()
        while not done:
            len_before = len(removed)
            removed |= remove_rolls(self.lines, removed)
            done = len_before == len(removed)
        return len(removed)
