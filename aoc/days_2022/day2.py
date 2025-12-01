from aoc.day import Day

SCORE = {
    'A': {
        'X': 3 + 1,
        'Y': 6 + 2,
        'Z': 0 + 3,
    },
    'B': {
        'X': 0 + 1,
        'Y': 3 + 2,
        'Z': 6 + 3,
    },
    'C': {
        'X': 6 + 1,
        'Y': 0 + 2,
        'Z': 3 + 3,
    }
}

STRAT = {
    'A': {
        'X': 'Z',
        'Y': 'X',
        'Z': 'Y',
    },
    'B': {
        'X': 'X',
        'Y': 'Y',
        'Z': 'Z',
    },
    'C': {
        'X': 'Y',
        'Y': 'Z',
        'Z': 'X'
    }
}


class Day2(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(2, test)

    def part1(self):
        return sum(SCORE[opponent][me]
                   for opponent, me in (line.split() for line in self.lines))

    def part2(self):
        return sum(SCORE[opponent][STRAT[opponent][strat]]
                   for opponent, strat in (line.split()
                                           for line in self.lines))
