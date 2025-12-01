from collections import defaultdict
from typing import Set

from aoc.day import Day

ALL_DIRS = [-1, -1 + 1j, 1j, 1 + 1j, 1, 1 - 1j, -1j, -1 - 1j]
CHECKS = [
    (-1 - 1j,  -1, -1 + 1j),
    (+1 - 1j,  +1, +1 + 1j),
    (-1 - 1j, -1j, +1 - 1j),
    (-1 + 1j, +1j, +1 + 1j),
]


class Day23(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(23, test)

    def part1(self):
        elves = self.__parse()
        offset = 0
        for _ in range(10):
            elves = self.__step(elves, offset)
            offset += 1
        min_row = int(min(i.real for i in elves))
        max_row = int(max(i.real for i in elves))
        min_col = int(min(i.imag for i in elves))
        max_col = int(max(i.imag for i in elves))
        return sum(1 for i in range(min_row, max_row + 1)
                   for j in range(min_col, max_col + 1)
                   if i + j * 1j not in elves)

    def part2(self):
        elves = self.__parse()
        new_elves = None
        i = 0
        while elves != new_elves:
            if new_elves:
                elves = new_elves
            new_elves = self.__step(elves, i)
            i += 1
        return i

    def __parse(self) -> Set[complex]:
        return set(i + j*1j for i, line in enumerate(self.lines)
                   for j, c in enumerate(line) if c == '#')

    def __step(self, elves: Set[complex], offset: int) -> Set[complex]:
        new_elves = set()
        proposed = defaultdict(int)
        moves = {}
        for elf in elves:
            if not any(elf + de in elves for de in ALL_DIRS):
                continue
            for i in range(len(CHECKS)):
                check = CHECKS[(offset + i) % len(CHECKS)]
                if not any(elf + de in elves for de in check):
                    new = elf + check[1]
                    proposed[new] += 1
                    moves[elf] = new
                    break
        for elf in elves:
            if elf not in moves:
                new_elves.add(elf)
                continue
            new = moves[elf]
            if proposed[new] == 1:
                new_elves.add(new)
            else:
                new_elves.add(elf)
        return new_elves
