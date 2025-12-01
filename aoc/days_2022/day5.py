from typing import List, Tuple
from aoc.day import Day


class Day5(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(5, test)

    def part1(self):
        stacks, bottom_idx = self.__get_stacks()

        for line in self.lines[bottom_idx + 1:]:
            amount, fro, to = self.__parse_instruction(line)

            for _ in range(amount):
                stacks[to].append(stacks[fro].pop())

        return "".join(s[-1] for s in stacks)

    def part2(self):
        stacks, bottom_idx = self.__get_stacks()

        for line in self.lines[bottom_idx + 1:]:
            amount, fro, to = self.__parse_instruction(line)

            stacks[to].extend(stacks[fro][-amount:])
            for _ in range(amount):
                stacks[fro].pop()

        return "".join(s[-1] for s in stacks)

    def __get_stacks(self) -> Tuple[List[List[str]], int]:
        bottom_idx = 0
        while self.lines[bottom_idx]:
            bottom_idx += 1
        num_stacks = len(self.lines[bottom_idx - 1].replace(" ", ""))
        stacks = [[] for _ in range(num_stacks)]

        for line in self.lines[bottom_idx - 2::-1]:
            i = 1
            stack_idx = 0
            while i < len(line):
                if line[i].isupper():
                    stacks[stack_idx].append(line[i])
                i += 4
                stack_idx += 1
        return stacks, bottom_idx

    def __parse_instruction(self, instruction: str) -> Tuple[int, int, int]:
        segments = instruction.split()
        amount = int(segments[1])
        fro = int(segments[3]) - 1
        to = int(segments[5]) - 1

        return amount, fro, to
