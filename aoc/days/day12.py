from collections import deque
from typing import Set

from aoc.days.day import Day
from aoc.util import get_grid_iter, get_grid_neighbours


class Day12(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(12, test)

    def part1(self):
        return self.__solve(set('S'), set('E'))

    def part2(self):
        # Both solutions work:
        # Solution 1:
        return self.__solve(set('E'), set(['S', 'a']), walk_down=True)
        # Solution 2:
        # return self.__solve(set(['S', 'a']), set('E'))

    def __solve(self, start_set: Set[str], end_set: Set[str],
                walk_down: bool = False,
                return_on_first: bool = True):
        Y = len(self.lines)
        X = len(self.lines[0])
        visited = set()
        q = deque()
        for y, x in get_grid_iter(Y, X, True):
            if self.lines[y][x] in start_set:
                visited.add((y, x))
                q.append(((y, x), 0))
        mul = -1 if walk_down else 1
        while len(q):
            (y, x), d = q.popleft()
            for ny, nx in get_grid_neighbours(Y, X, y, x):
                if (ny, nx) in visited \
                        or mul * (self.__height(ny, nx)
                                  - self.__height(y, x)) > 1:
                    continue
                if self.lines[ny][nx] in end_set:
                    return d + 1
                visited.add((ny, nx))
                q.append(((ny, nx), d + 1))
        raise ValueError("Unsolvable maze!")

    def __height(self, y: int, x: int):
        c = self.lines[y][x]
        return 0 if c == 'S' else 25 if c == 'E' else ord(c) - ord('a')
