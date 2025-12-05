from aoc.day import Day


def merge_ranges(ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """
    Note: expects merges to be sorted by start
    """
    merged_ranges = []
    to_merge = None
    for range in ranges:
        if to_merge is None:
            to_merge = range
            continue
        if to_merge[0] <= range[0] <= to_merge[1]:
            to_merge = (to_merge[0], max(to_merge[1], range[1]))
        else:
            merged_ranges.append(to_merge)
            to_merge = range
    merged_ranges.append(to_merge)
    return merged_ranges


class Day5(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(5, test)

    def part1(self):
        empty_line_idx = next(i for i, e in enumerate(self.lines) if not e)
        ranges, elements = self.lines[:empty_line_idx], self.lines[empty_line_idx + 1 :]

        ranges = sorted(
            [(int(parts[0]), int(parts[1])) for r in ranges for parts in [r.split("-")]]
        )
        elements = list(map(int, elements))

        return sum(any(r[0] <= element <= r[1] for r in ranges) for element in elements)

    def part2(self):
        empty_line_idx = next(i for i, e in enumerate(self.lines) if not e)
        ranges, _ = self.lines[:empty_line_idx], self.lines[empty_line_idx + 1 :]

        ranges = sorted(
            [(int(parts[0]), int(parts[1])) for r in ranges for parts in [r.split("-")]]
        )
        ranges = merge_ranges(ranges)
        return sum(r[1] - r[0] + 1 for r in ranges)
