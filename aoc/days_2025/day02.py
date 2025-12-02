from aoc.day import Day


def _is_id_invalid(id: int, max_repeats: int = 2) -> bool:
    id_str = str(id)

    for repeats in range(2, min(max_repeats, len(id_str)) + 1):
        if len(id_str) % repeats:
            continue
        invalid = True
        segment_len = len(id_str) // repeats
        for repeat_num in range(repeats - 1):
            if (
                id_str[(repeat_num) * segment_len : (repeat_num + 1) * segment_len]
                != id_str[
                    (repeat_num + 1) * segment_len : (repeat_num + 2) * segment_len
                ]
            ):
                invalid = False
                break
        if invalid:
            return True
    return False


class Day2(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(2, test)

    def _solve(self, max_repeats):
        ans = 0
        for ids in self.lines[0].split(","):
            start, end = ids.split("-")
            for id in range(int(start), int(end) + 1):
                ans += id if _is_id_invalid(id, max_repeats) else 0
        return ans

    def part1(self):
        return self._solve(2)

    def part2(self):
        return self._solve(int(2e10))
