class Day(object):
    def __init__(self, num: int, test: bool) -> None:
        self.num = num
        self.__init_lines(test)

    def part1(self):
        raise NotImplementedError

    def part2(self):
        raise NotImplementedError

    def __init_lines(self, test: bool):
        with open(f"in/{self.num:0>2}{'_test' if test else ''}.in") as f:
            self.lines = f.read().splitlines()
