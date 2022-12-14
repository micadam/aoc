from aoc.days.day import Day

DPS = [(0, 1), (-1, 1), (1, 1), (0, 0)]


class Day14(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(14, test)

    def part1(self):
        return self.__simulate()

    def part2(self):
        return self.__simulate(floor=True)

    def __simulate(self, floor: bool = False):
        grains = set()
        obstacles = set()
        for line in self.lines:
            first, *pairs = line.split("->")
            x, y = map(int, first.split(","))
            for pair in pairs:
                nx, ny = map(int, pair.split(","))
                if not nx - x:
                    obstacles.update((x, py) for py
                                     in range(min(ny, y), max(ny, y) + 1))
                elif not ny - y:
                    obstacles.update((px, y) for px
                                     in range(min(nx, x), max(nx, x) + 1))
                x, y = nx, ny
        max_y = max(y for _, y in obstacles)
        if floor:
            max_y = max_y + 2
        while True:
            x, y = 500, 0
            idx = 0
            still = False
            while not still:
                if idx == len(DPS):
                    if floor and (x, y) == (500, 0):
                        return len(grains)
                    else:
                        raise ValueError("Out of movement.")
                dx, dy = DPS[idx]
                nx, ny = x + dx, y + dy
                if not (nx, ny) in grains and \
                        not (nx, ny) in obstacles \
                        and not (floor and ny == max_y):
                    if not (dx or dy):
                        grains.add((nx, ny))
                        still = True
                    else:
                        x, y = nx, ny
                        idx = 0
                else:
                    idx += 1
                if y > max_y:
                    return len(grains)
