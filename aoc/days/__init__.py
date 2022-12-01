from typing import Dict, Type

from aoc.days.day import Day
from aoc.days.day1 import Day1

DAYS: Dict[int, Type[Day]] = {
    1: Day1
}

__all__ = ['DAYS']
