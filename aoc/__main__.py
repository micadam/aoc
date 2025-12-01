import argparse

from aoc.days_2022 import DAYS as DAYS_2022
from aoc.days_2025 import DAYS as DAYS_2025
from aoc.util import time_method

DAYS_PER_YEAR = {
    2022: DAYS_2022,
    2025: DAYS_2025,
}
parser = argparse.ArgumentParser(prog="aoc2022")
parser.add_argument("day_numbers", type=int, nargs="*")
parser.add_argument("-p", "--parts", nargs="*", default=[1, 2], type=int)
parser.add_argument("-y", "--year", default=2025, type=int)
parser.add_argument("-t", "--test_mode", action="store_true")

args = parser.parse_args()

if args.day_numbers:
    day_numbers = args.day_numbers
else:
    day_numbers = DAYS_PER_YEAR[args.year].keys()

for day_number in day_numbers:
    if day_number not in DAYS_PER_YEAR[args.year]:
        raise ValueError(f"Day {day_number} does not exist for year {[args.year]}!")

test = args.test_mode
for day_number in day_numbers:
    day = DAYS_PER_YEAR[args.year][day_number](test)  # type: ignore

    print(f"Running day {day_number}.")
    if 1 in args.parts:
        ans, t = time_method(day.part1)
        print(f"Part 1: {ans}, took {t:.5f} s")
    if 2 in args.parts:
        ans, t = time_method(day.part2)
        print(f"Part 2: {ans}, took {t:.5f} s")
