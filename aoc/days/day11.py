import heapq
import operator
from math import lcm
from typing import List, Tuple

from aoc.days.day import Day

OPS = {
    "+": operator.add,
    "*": operator.mul,
}


class Monkey:
    def __init__(self, description: List[str], divide: bool):
        description = [line.strip() for line in description]
        assert description[0].startswith("Monkey ")
        assert description[1].startswith("Starting items: ")
        assert description[2].startswith("Operation: new = ")
        assert description[3].startswith("Test: divisible by ")
        assert description[4].startswith("If true: throw to monkey ")
        assert description[5].startswith("If false: throw to monkey ")
        assert not description[6]
        self.items = list(map(int, description[1].split(maxsplit=2)[-1]
                              .split(", ")))
        op_segments = description[2].split()
        self.operand1 = int(op_segments[3]) \
            if op_segments[3].isdigit() \
            else op_segments[3]
        self.operation = OPS[op_segments[4]]
        self.operand2 = int(op_segments[5]) \
            if op_segments[5].isdigit() \
            else op_segments[5]
        self.test_val = int(description[3].split()[-1])
        self.true_monkey = int(description[4].split()[-1])
        self.false_monkey = int(description[5].split()[-1])
        self.score = 0
        self.divide = divide

    def process(self) -> List[Tuple[int, int]]:
        to_throw = []
        for item in self.items:
            self.score += 1
            operand1 = self.operand1 if self.operand1 != "old" else item
            operand2 = self.operand2 if self.operand2 != "old" else item
            new = self.operation(operand1, operand2)
            if self.divide:
                new = new // 3
            addressee = self.true_monkey \
                if not new % self.test_val \
                else self.false_monkey
            to_throw.append((new, addressee))
        self.items = []
        return to_throw

    def add_item(self, item: int):
        self.items.append(item)

    def clean_up(self, lcm):
        self.items = [i % lcm for i in self.items]


class Day11(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(11, test)
        if self.lines[-1]:
            self.lines.append("")

    def part1(self):
        return self.__monkey_fun(20, divide=True)

    def part2(self):
        return self.__monkey_fun(10000, divide=False)

    def __monkey_fun(self, num_rounds: int, divide: bool):
        monkeys: List[Monkey] = []
        for i in range(0, len(self.lines), 7):
            monkeys.append(Monkey(self.lines[i:i + 7],
                                  divide=divide))
        lowest_common_multiple = lcm(*[m.test_val for m in monkeys])
        for i in range(num_rounds):
            for monkey in monkeys:
                to_throw = monkey.process()
                for item, addressee in to_throw:
                    monkeys[addressee].add_item(item)
            if not divide:
                for monkey in monkeys:
                    monkey.clean_up(lowest_common_multiple)
        a, b = heapq.nlargest(2, (m.score for m in monkeys))
        return a * b
