import shapely

from aoc.day import Day


class Day9(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(9, test)

    def part1(self):
        points = [
            (int(nums[0]), int(nums[1]))
            for nums in [line.split(",") for line in self.lines]
        ]
        return max(
            (abs(p1[0] - p2[0]) + 1) * (abs(p1[1] - p2[1]) + 1)
            for p1 in points
            for p2 in points
        )

    def part2(self):
        points = [
            (int(nums[0]), int(nums[1]))
            for nums in [line.split(",") for line in self.lines]
        ]

        points.append(points[0])

        polygon = shapely.Polygon(points)
        return max(
            (abs(p1[0] - p2[0]) + 1) * (abs(p1[1] - p2[1]) + 1)
            for p1 in points
            for p2 in points
            if polygon.contains(shapely.box(p1[0], p1[1], p2[0], p2[1]))
        )
