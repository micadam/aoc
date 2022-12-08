import argparse

from aoc.days import DAYS
from aoc.util import time_method


parser = argparse.ArgumentParser(prog="aoc2022")
parser.add_argument("day_number", type=int)
parser.add_argument("-p", "--parts", nargs="*", default=[1, 2], type=int)
parser.add_argument("-t", "--test_mode", action='store_true')

args = parser.parse_args()

if args.day_number not in DAYS:
    raise ValueError(f"Day {args.day_number} does not exist!")

test = args.test_mode
day = DAYS[args.day_number](test)

print(f"Running day {args.day_number}.")
if 1 in args.parts:
    ans, t = time_method(day.part1)
    print(f"Part 1: {ans}, took {t:.5f} s")
if 2 in args.parts:
    ans, t = time_method(day.part2)
    print(f"Part 2: {ans}, took {t:.5f} s")
