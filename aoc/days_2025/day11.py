from collections import defaultdict, deque
import functools
from aoc.day import Day

PATHS = {}


@functools.cache
def num_paths_from(from_node: str, dac_reached: bool, fft_reached: bool):
    if from_node == "out":
        return int(dac_reached and fft_reached)
    if from_node == "fft":
        fft_reached = True
    if from_node == "dac":
        dac_reached = True

    return sum(
        num_paths_from(to_node, dac_reached, fft_reached)
        for to_node in PATHS[from_node]
    )


class Day11(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(11, test)

    def parse(self):
        return {(s := line.split(": "))[0]: s[1].split() for line in self.lines}

    def part1(self):
        paths = self.parse()

        q = deque(["you"])
        ans = 0

        while q:
            current = q.popleft()
            if current == "out":
                ans += 1
                continue
            for next in paths[current]:
                q.append((next))
        return ans

    def part2(self):
        paths = self.parse()
        global PATHS
        PATHS = paths

        return num_paths_from("svr", False, False)
