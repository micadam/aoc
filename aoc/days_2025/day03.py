import numpy as np

from aoc.day import Day


class Day3(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(3, test)

    def _solve(self, num_batteries=2):
        volts = np.array([list(map(int, line)) for line in self.lines])
        batteries = []
        mask = np.ones_like(volts)
        for i in range(num_batteries):
            masked_volts = np.where(mask, volts, -np.inf)
            batteries_after_this_one = num_batteries - i - 1
            battery = np.argmax(
                masked_volts[:, : volts.shape[1] - batteries_after_this_one], axis=1
            )
            batteries.append(volts[np.arange(volts.shape[0]), battery])
            mask = np.arange(volts.shape[1]) > battery[:, None]

        return sum(
            10 ** (num_batteries - i - 1) * battery.sum()
            for i, battery in enumerate(batteries)
        )

    def part1(self):
        return self._solve()

    def part2(self):
        return self._solve(12)
