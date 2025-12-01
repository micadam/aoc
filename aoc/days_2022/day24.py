from collections import deque
from functools import cache
from math import lcm
from typing import Deque, Tuple

from aoc.day import Day

DIRS = {
    1: '^',
    1j: '<',
    -1: 'v',
    -1j: '>',
    0: None,
}


class Day24(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(24, test)

    def part1(self):
        return self.min_t(False)

    def part2(self):
        return self.min_t(True)

    def min_t(self, need_snacks: bool):
        needed_points = 3 if need_snacks else 1
        self.__safe_at_time.cache_clear()
        self.grid = self.__get_grid()
        self.max_row = int(max(i.real for i in self.grid))
        self.max_col = int(max(i.imag for i in self.grid))
        mod = lcm(self.max_row - 1, self.max_col - 1)
        start_pos = next(k for k, v in self.grid.items()
                         if k.real == 0 and v == '.')
        end_pos = next(k for k, v in self.grid.items()
                       if k.real == self.max_row and v == '.')
        q: Deque[Tuple[complex, int, int]] = deque([(start_pos, 0, 0)])
        visited = set(q)
        while q:
            pos, t, points = q.popleft()
            goal = start_pos if points % 2 else end_pos
            if pos == goal:
                if points + 1 == needed_points:
                    return t
                else:
                    points += 1
            for dpos in DIRS:
                new_pos = pos + dpos
                if new_pos in self.grid and self.grid[new_pos] != '#' \
                        and self.__safe_at_time(new_pos, (t + 1) % mod) \
                        and not (new_pos, (t + 1) % mod, points) in visited:
                    visited.add((new_pos, (t + 1) % mod, points))
                    q.append((new_pos, t + 1, points))
        raise RuntimeError("Unsolvable.")

    def __get_grid(self):
        grid = {}
        for row, line in enumerate(self.lines):
            for col, c in enumerate(line):
                grid[row + col * 1j] = c
        return grid

    @cache
    def __safe_at_time(self, pos, t):
        # Assume no blizzards ever enter the top and bottom rows.
        # This is true for both the test input and my input
        if pos.real == 0 or pos.real == self.max_row:
            return True
        for dpos, scary_wind in DIRS.items():
            if not scary_wind:
                continue
            new_real = (pos.real - 1 + t * dpos.real) % (self.max_row - 1) + 1
            new_imag = (pos.imag - 1 + t * dpos.imag) % (self.max_col - 1) + 1
            new_pos = new_real + new_imag * 1j
            if self.grid[new_pos] == scary_wind:
                return False
        return True
