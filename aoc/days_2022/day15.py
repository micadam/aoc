from aoc.day import Day


class Day15(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(15, test)
        self.test = test
        self.s = []
        for line in self.lines:
            segments = line.split()
            sx = int(segments[2].split("=")[1][:-1])
            sy = int(segments[3].split("=")[1][:-1])
            bx = int(segments[8].split("=")[1][:-1])
            by = int(segments[9].split("=")[1])
            self.s.append(((sx, sy),
                           abs(sx - bx) + abs(sy - by)))
        self.s.sort()

    def part1(self):
        pos = set()
        goal = 10 if self.test else 2000000

        for (sx, sy), d in self.s:
            arm = d - abs(sy - goal)
            if arm <= 0:
                continue
            pos.update(range(sx - arm, sx + arm))
        return len(pos)

    def part2(self):
        max_coord = 20 if self.test else 4000000
        x, y = 0, 0
        moved = True
        while moved and x <= max_coord and y <= max_coord:
            moved = False
            for (sx, sy), d in self.s:
                arm = d - abs(y - sy)
                if arm <= 0 or sx - arm > x or sx + arm < x:
                    continue
                x = sx + arm + 1
                if x > max_coord:
                    x, y = 0, y + 1
                moved = True
        return 4000000 * x + y
