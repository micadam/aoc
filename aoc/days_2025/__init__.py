from typing import Dict, Type

from aoc.day import Day
from .day01 import Day1
from .day02 import Day2
from .day03 import Day3
from .day04 import Day4
from .day05 import Day5
from .day06 import Day6
from .day07 import Day7
from .day08 import Day8
from .day09 import Day9
from .day10 import Day10
from .day11 import Day11
from .day12 import Day12

DAYS: Dict[int, Type[Day]] = {
    1: Day1,
    2: Day2,
    3: Day3,
    4: Day4,
    5: Day5,
    6: Day6,
    7: Day7,
    8: Day8,
    9: Day9,
    10: Day10,
    11: Day11,
    12: Day12,
}

__all__ = ['DAYS']
