from aoc.days.day import Day
from aoc.util import Pos

DIRS = {
    'U': Pos(0, 1),
    'D': Pos(0, -1),
    'L': Pos(-1, 0),
    'R': Pos(1, 0)
}


class Day9(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(9, test)

    def part1(self):
        return self.__simulate(2)

    def part2(self):
        return self.__simulate(10)

    def __simulate(self, length):
        commands = map(str.split, self.lines)
        positions = set([Pos(0, 0)])

        rope = [Pos(0, 0) for _ in range(length)]
        for dir, amt in commands:
            amt = int(amt)
            dpos = DIRS[dir]
            for _ in range(amt):
                rope[0] += dpos
                for i in range(1, length):
                    diff = rope[i-1] - rope[i]
                    if max(abs(diff.x), abs(diff.y)) <= 1:
                        continue
                    rope[i] += diff.binary_dir()
                positions.add(rope[-1])
        return len(positions)
