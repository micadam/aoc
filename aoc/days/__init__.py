from typing import Dict, Type

from aoc.days.day import Day
from aoc.days.day1 import Day1
from aoc.days.day2 import Day2

DAYS: Dict[int, Type[Day]] = {
    1: Day1,
    2: Day2,
}

__all__ = ['DAYS']
