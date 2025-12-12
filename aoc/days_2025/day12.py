from aoc.day import Day


class Day12(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(12, test)

    def parse(self):
        presents = {}
        while (line := self.lines.pop(0)).endswith(":"):
            prsent_id = int(line.split(":")[0])
            present_lines = self.lines[:3]
            self.lines = self.lines[4:]  # skip empty lines as well
            presents[prsent_id] = present_lines

        trees = []
        for line in self.lines:
            size, present_ids = line.split(": ")
            width, length = size.split("x")
            width, length = int(width), int(length)
            present_ids = [int(id) for id in present_ids.split()]
            assert len(present_ids) == len(presents)
            trees.append((width, length, present_ids))

        return presents, trees

    def part1(self):
        _, trees = self.parse()
        # Just give each present a full 3x3 box I guess.
        return sum(
            (width // 3) * (length // 3) > sum(presents)
            for width, length, presents in trees
        )

    def part2(self):
        return "Merry Christmas!"
