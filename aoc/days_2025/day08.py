import functools
import itertools
import math
from typing import Any

from aoc.day import Day


@functools.cache
def euclidean_distance(a: tuple[int, int, int], b: tuple[int, int, int]):
    return math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2)


def size_of_three_largest_sets(sets: dict[Any, set]):
    largest_sets = sorted(sets.values(), key=len, reverse=True)
    return (len(largest_sets[0]), len(largest_sets[1]), len(largest_sets[2]))


class Day8(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(8, test)

    def part1(self):
        boxes: list[tuple[int, int, int]] = [
            tuple(int(x) for x in line.split(",")) for line in self.lines  # type: ignore
        ]
        closest_pairs = sorted(
            [(i, j) for i in range(len(boxes)) for j in range(i + 1, len(boxes))],
            key=lambda x: euclidean_distance(boxes[x[0]], boxes[x[1]]),
        )
        sets = {i: {i} for i in range(len(boxes))}
        set_ids = {i: i for i in range(len(boxes))}
        for idx in range(1000):
            i, j = closest_pairs[idx]
            if set_ids[i] == set_ids[j]:
                continue
            sets[set_ids[i]] |= sets[set_ids[j]]
            to_change = sets[set_ids[j]]
            del sets[set_ids[j]]
            for box in to_change:
                set_ids[box] = set_ids[i]
        largest_lens = size_of_three_largest_sets(sets)
        return largest_lens[0] * largest_lens[1] * largest_lens[2]

    def part2(self):
        boxes: list[tuple[int, int, int]] = [
            tuple(int(x) for x in line.split(",")) for line in self.lines  # type: ignore
        ]
        closest_pairs = sorted(
            [(i, j) for i in range(len(boxes)) for j in range(i + 1, len(boxes))],
            key=lambda x: euclidean_distance(boxes[x[0]], boxes[x[1]]),
        )
        sets = {i: {i} for i in range(len(boxes))}
        set_ids = {i: i for i in range(len(boxes))}
        ans = None
        idx = 0
        while len(sets) > 1:
            i, j = closest_pairs[idx]
            idx += 1
            if set_ids[i] == set_ids[j]:
                continue
            ans = boxes[i][0] * boxes[j][0]
            sets[set_ids[i]] |= sets[set_ids[j]]
            to_change = sets[set_ids[j]]
            del sets[set_ids[j]]
            for box in to_change:
                set_ids[box] = set_ids[i]
        return ans
