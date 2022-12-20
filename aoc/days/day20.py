from typing import List, Tuple
from aoc.days.day import Day


NUMS_TYPE = List[Tuple[int, int]]


class Day20(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(20, test)

    def part1(self):
        nums = self.__get_nums()
        nums = self.__mix(nums)
        return self.__get_answer(nums)

    def part2(self):
        nums = self.__get_nums(811589153)
        for _ in range(10):
            nums = self.__mix(nums)
        return self.__get_answer(nums)

    def __get_nums(self, mul: int = 1) -> NUMS_TYPE:
        return [(i, int(num) * mul) for i, num in enumerate(self.lines)]

    def __mix(self, nums: NUMS_TYPE) -> NUMS_TYPE:
        for i in range(len(nums)):
            idx = next(j for j, (k, _) in enumerate(nums) if k == i)
            num = nums[idx]
            nums = nums[:idx] + nums[idx + 1:]
            new_idx = (idx + num[1]) % len(nums)
            nums = nums[:new_idx] + [num] + nums[new_idx:]
        return nums

    def __get_answer(self, nums: NUMS_TYPE) -> int:
        zero_idx = next(j for j, (_, e) in enumerate(nums) if e == 0)
        return sum(nums[(zero_idx + idx) % len(nums)][1] for idx in [1000,
                                                                     2000,
                                                                     3000])
