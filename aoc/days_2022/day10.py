from aoc.day import Day

DURATION = {
    "noop": 1,
    "addx": 2
}


class Day10(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(10, test)

    def part1(self):
        cmds = list(map(str.split, self.lines))
        clock = 1
        next_key_time = 20
        score = 0
        X = 1
        for cmd in cmds:
            op = cmd[0]
            val = int(cmd[1] if op.startswith("addx") else 0)
            dur = DURATION[op]
            new_time = clock + dur
            if new_time > next_key_time:
                score += next_key_time * X
                next_key_time += 40
            X += val
            clock = new_time
            if clock >= 220:
                break
        return score

    def part2(self):
        cmds = list(map(str.split, self.lines))
        clock = 0
        X = 1
        for cmd in cmds:
            op = cmd[0]
            val = int(cmd[1] if op.startswith("addx") else 0)
            dur = DURATION[op]
            for i in range(dur):
                if (clock + i) > 0 and not (clock + i) % 40:
                    print()
                    clock -= 40
                print('#' if abs(clock + i - X) <= 1 else '.', end="")
            clock += dur
            X += val
        print()
