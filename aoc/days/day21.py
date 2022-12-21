from operator import add, floordiv, mul, sub
from typing import Dict, Literal, Optional, Tuple

from aoc.days.day import Day

CMDS = {
    '+': add,
    '-': sub,
    '*': mul,
    '/': floordiv
}

RV_CMDS = {
    '+': sub,
    '-': add,
    '*': floordiv,
    '/': mul,
    '=': lambda a, b: a or b,
}

VALS_TYPE = Dict[str, Optional[int]]
CMDS_TYPE = Dict[str, Tuple[str, Literal['+', '-', '*', '/'], str]]

HUMN = "humn"
ROOT = "root"


class Day21(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(21, test)

    def part1(self):
        vals, cmds = self.__parse_monkeys()
        return self.__get_val(ROOT, vals, cmds)

    def part2(self):
        vals, cmds = self.__parse_monkeys()
        return self.__find_unk_val(None, ROOT, vals, cmds)

    def __parse_monkeys(self) -> Tuple[VALS_TYPE, CMDS_TYPE]:
        vals = {}
        cmds = {}
        for line in self.lines:
            monkey, rest = line.split(": ")
            if rest.isdigit():
                vals[monkey] = int(rest)
            else:
                op1, cmd, op2 = rest.split()
                cmds[monkey] = (op1, cmd, op2)
        return vals, cmds

    def __get_val(self, monkey: str,
                  vals: VALS_TYPE, cmds: CMDS_TYPE,
                  accept_humn: bool = True) -> Optional[int]:
        if monkey == HUMN and not accept_humn:
            vals[monkey] = None
            return None
        if monkey not in vals:
            op1, cmd, op2 = cmds[monkey]
            val1 = self.__get_val(op1, vals, cmds, accept_humn)
            val2 = self.__get_val(op2, vals, cmds, accept_humn)
            if None in (val1, val2):
                vals[monkey] = None
                return None
            vals[monkey] = CMDS[cmd](val1, val2)
        return vals[monkey]

    def __find_unk_val(self, parent_val: Optional[int], monkey: str,
                       vals: VALS_TYPE, cmds: CMDS_TYPE) -> int:
        if monkey == HUMN:
            assert parent_val is not None
            return parent_val
        op1, cmd, op2 = cmds[monkey]
        if monkey == ROOT:
            cmd = '='
        val1 = self.__get_val(op1, vals, cmds, accept_humn=False)
        val2 = self.__get_val(op2, vals, cmds, accept_humn=False)
        if cmd in ('-', '/') and val1 is not None:
            new_parent_val = CMDS[cmd](val1, parent_val)
        elif val1 is not None:
            new_parent_val = RV_CMDS[cmd](parent_val, val1)
        elif val2 is not None:
            new_parent_val = RV_CMDS[cmd](parent_val, val2)
        else:
            raise ValueError("Both val1 and val2 are None")
        new_monkey = op1 if val2 is not None else op2
        return self.__find_unk_val(new_parent_val, new_monkey, vals, cmds)
