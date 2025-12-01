from aoc.day import Day


class Day3(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(3, test)

    def part1(self):
        return sum(
            self.__get_priority(next(
                i for i in rucksack
                if i in rucksack[:(size := len(rucksack) // 2)]
                and i in rucksack[size:]))
            for rucksack in self.lines
        )

    def part2(self):
        return sum(
            self.__get_priority(next(
                c for c in self.lines[i]
                if all(c in r for r in self.lines[i + 1:i + 3])))
            for i in range(0, len(self.lines), 3)
        )

    def __get_priority(self, char: str):
        if char.islower():
            return ord(char) - ord('a') + 1
        if char.isupper():
            return ord(char) - ord('A') + 27
        raise ValueError(f"Not a letter: {char}")
