from collections import deque
from scipy import optimize

import numpy as np
from aoc.day import Day


class Day10(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(10, test)

    def bfs(
        self,
        initial_state,
        final_state,
        buttons,
        button_effect,
    ) -> int:
        q = deque([(initial_state, 0)])
        visited = set()

        while q:
            state, presses = q.popleft()
            if state == final_state:
                return presses

            for button in buttons:
                new_state = button_effect(state, button)
                if new_state not in visited:
                    visited.add(new_state)
                    q.append((new_state, presses + 1))
        raise ValueError(f"Unreachable state: {final_state} {buttons}")

    def min_presses(self, line: str) -> int:
        final_state, line = line.split(maxsplit=1)
        final_state = tuple(
            True if c == "#" else False for c in final_state.strip("[]")
        )

        buttons = [
            set(int(segment) for segment in button.strip("()").split(","))
            for button in line.split()[:-1]
        ]

        initial_state = tuple(False for _ in range(len(final_state)))

        button_effect = lambda state, button: tuple(
            s if i not in button else not s for i, s in enumerate(state)
        )
        return self.bfs(initial_state, final_state, buttons, button_effect)

    def min_presses_joltage(self, line):
        _, line = line.split(maxsplit=1)
        buttons, joltages = line.rsplit(maxsplit=1)

        buttons = [
            set(int(segment) for segment in button.strip("()").split(","))
            for button in line.split()[:-1]
        ]
        joltages = [int(joltage) for joltage in joltages.strip("{}").split(",")]
        max_button_presses = max(joltages)

        constraints = []
        for i in range(len(joltages)):
            constraints.append(
                [1 if i in buttons[j] else 0 for j in range(len(buttons))]
            )
        result = optimize.linprog(
            np.ones(len(buttons)),
            A_eq=constraints,
            b_eq=joltages,
            bounds=[(0, max_button_presses) for _ in range(len(buttons))],
            integrality=np.ones(len(buttons)),
        )
        if not result.success or result.fun - round(result.fun) > 1e-2:
            raise ValueError(f"Unsolvable: {result['message']}, {result.fun % 1}")
        return round(result.fun)

    def part1(self):
        return sum(self.min_presses(line) for line in self.lines)

    def part2(self):
        return sum(self.min_presses_joltage(line) for line in self.lines)
