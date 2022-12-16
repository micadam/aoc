from collections import deque
from functools import cache
from itertools import product

from aoc.days.day import Day


class Day16(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(16, test)
        self.dests = {}
        self.pressure = {}
        for line in self.lines:
            segments = line.split()
            name = segments[1]
            self.pressure[name] = int(segments[4].split("=")[1][:-1])
            self.dests[name] = [s.replace(',', '') for s in segments[9:]]
        self.costs = {}
        # BFS from every node to find travel times
        # This should be the Floyd-Warshall algorithm
        # but I didn't remember it at 5 AM
        for valve, dest in self.dests.items():
            visited = set()
            to_visit = deque([(d, 1) for d in dest])
            self.costs[valve] = {}
            while to_visit:
                d, cost = to_visit.popleft()
                if d in visited:
                    continue
                visited.add(d)
                self.costs[valve][d] = cost
                to_visit.extend((new_d, cost + 1)
                                for new_d in self.dests[d])
        self.active_valves = frozenset(valve for valve, p
                                       in self.pressure.items() if p)

    def part1(self):
        return self.dfs(30, self.active_valves)

    def part2(self):
        active_valves_list = list(self.active_valves)
        return max(sum(self.dfs(26, frozenset(v for i, v
                                              in enumerate(active_valves_list)
                                              if a[i] == w))
                       for w in range(2))
                   for a in product(range(2), repeat=len(self.active_valves)))

    @cache
    def dfs(self,
            T: int,
            active_valves: frozenset,
            current_valve: str = 'AA'):
        return max((new_T * self.pressure[v] + self.dfs(new_T,
                                                        active_valves - {v},
                                                        v)
                    for v in active_valves
                    if (new_T := T - self.costs[current_valve][v] - 1) > 0),
                   default=0)
