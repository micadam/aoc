from collections import deque

from aoc.days.day import Day

DPOS = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1)
]


class Day18(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(18, test)

    def part1(self):
        cubes = set()
        for line in self.lines:
            cubes.add(tuple(map(int, line.split(","))))
        ans = 0
        for a, b, c in cubes:
            for da, db, dc in DPOS:
                ans += ((a + da, b + db, c + dc) not in cubes)
        return ans

    def part2(self):
        cubes = set()
        groups = {}
        for line in self.lines:
            cubes.add(tuple(map(int, line.split(","))))
        a_range = range(min(na for na, _, _ in cubes) - 1,
                        max(na for na, _, _ in cubes) + 2)
        b_range = range(min(nb for _, nb, _ in cubes) - 1,
                        max(nb for _, nb, _ in cubes) + 2)
        c_range = range(min(nc for _, _, nc in cubes) - 1,
                        max(nc for _, _, nc in cubes) + 2)
        group = 0
        visited = set()
        for a in a_range:
            for b in b_range:
                for c in c_range:
                    if (a, b, c) in cubes or (a, b, c) in visited:
                        continue
                    group += 1
                    to_visit = deque([(a, b, c)])
                    while to_visit:
                        na, nb, nc = to_visit.popleft()
                        if ((na, nb, nc)) in visited:
                            continue
                        visited.add((na, nb, nc))
                        groups[(na, nb, nc)] = group
                        for da, db, dc in DPOS:
                            nna, nnb, nnc = na + da, nb + db, nc + dc
                            if nna not in a_range \
                                    or nnb not in b_range \
                                    or nnc not in c_range \
                                    or (nna, nnb, nnc) in cubes \
                                    or (nna, nnb, nnc) in to_visit:
                                continue
                            to_visit.append((nna, nnb, nnc))
        ans = 0
        for a, b, c in cubes:
            for da, db, dc in DPOS:
                cube = (a + da, b + db, c + dc)
                ans += (cube not in cubes and groups[cube] == 1)
        return ans
