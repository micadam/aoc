from functools import cmp_to_key
from typing import List, Literal, Union

from aoc.days.day import Day


class Signal:
    def __init__(self, line: Union[str, List]):
        if isinstance(line, list):
            self.items = line
            return
        self.items = []
        items_raw = line[1:-1]
        item_start = item_end = 0
        depth = 0
        while item_end <= len(items_raw):
            if item_end == len(items_raw) and depth > 0:
                raise ValueError("Unclosed bracket.")
            if item_end == len(items_raw) \
                    or items_raw[item_end] == ',' and depth == 0:
                item_raw = items_raw[item_start:item_end]
                item_start = item_end = item_end + 1
                if item_raw.isdigit():
                    self.items.append(int(item_raw))
                elif not item_raw:
                    continue
                else:
                    self.items.append(Signal(item_raw))
                continue
            elif items_raw[item_end] == '[':
                depth += 1
            elif items_raw[item_end] == ']':
                depth -= 1
            item_end += 1
        assert item_start == item_end == len(items_raw) + 1

    def __eq__(self, __o: object) -> bool:
        if not isinstance(__o, Signal):
            return False
        return self.items == __o.items

    def __repr__(self) -> str:
        return f"S{self.items}"


def cmp(ll, rr) -> Literal[-1, 0, 1]:
    if ll == rr:
        return 0
    l_is_signal = isinstance(ll, Signal)
    r_is_signal = isinstance(rr, Signal)
    if not l_is_signal and not r_is_signal:
        return -1 if ll < rr \
            else 1 if ll > rr \
            else 0
    if not l_is_signal:
        ll = Signal([ll])
    if not r_is_signal:
        rr = Signal([rr])
    for i in range(max(len(ll.items), len(rr.items))):
        if i >= len(ll.items):
            return -1
        if i >= len(rr.items):
            return 1
        rec_result = cmp(ll.items[i], rr.items[i])
        if rec_result in (-1, 1):
            return rec_result
    return 0


class Day13(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(13, test)

    def part1(self):
        ans = 0
        for i in range(0, len(self.lines), 3):
            ll = Signal(self.lines[i])
            rr = Signal(self.lines[i + 1])

            res = cmp(ll, rr)
            assert res != 0
            if res == -1:
                ans += i // 3 + 1
        return ans

    def part2(self):
        signals = []
        for line in self.lines:
            if not line:
                continue
            signals.append(Signal(line))
        signals.sort(key=cmp_to_key(cmp))
        dividers = [
            Signal("[[2]]"),
            Signal("[[6]]")
        ]
        ans = 1
        for divider in dividers:
            i = 0
            while cmp(signals[i], divider) == -1:
                i += 1
            signals.insert(i, divider)
        for divider in dividers:
            idx = next(i for i in range(len(signals)) if signals[i] is divider)
            ans *= idx + 1
        return ans
