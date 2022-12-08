from typing import List
from aoc.days.day import Day
from aoc.util import get_grid_iter


class Day8(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(8, test)
        self.ma = -1
        self.height_idx = {}

    def part1(self):
        Y = len(self.lines)
        X = len(self.lines[0])
        grid = [list(map(int, row)) for row in self.lines]
        vis_grid = [[False for _ in range(X)] for _ in range(Y)]
        self.__fill_vis_grid(grid, vis_grid, Y, X, True)
        self.__fill_vis_grid(grid, vis_grid, Y, X, True, reverse_X=True)
        self.__fill_vis_grid(grid, vis_grid, Y, X, False)
        self.__fill_vis_grid(grid, vis_grid, Y, X, False, reverse_Y=True)
        return sum(elt for row in vis_grid for elt in row if elt)

    def __fill_vis_grid(self,
                        grid: List[List[int]], vis_grid: List[List[bool]],
                        Y: int, X: int, row_first: bool,
                        reverse_Y: bool = False,
                        reverse_X: bool = False) -> None:
        for y, x in get_grid_iter(Y, X, row_first,
                                  reverse_Y=reverse_Y, reverse_X=reverse_X,
                                  new_loop_handler=self.__reset_ma):
            elt = grid[y][x]
            if elt > self.ma:
                self.ma = elt
                vis_grid[y][x] = True

    def __reset_ma(self):
        self.ma = -1

    def part2(self):
        Y = len(self.lines)
        X = len(self.lines[0])
        grid = [list(map(int, row)) for row in self.lines]
        scores = [[1 for _ in range(X)] for _ in range(Y)]
        self.__update_vis_scores(grid, scores, Y, X, True)
        self.__update_vis_scores(grid, scores, Y, X, True, reverse_X=True)
        self.__update_vis_scores(grid, scores, Y, X, False)
        self.__update_vis_scores(grid, scores, Y, X, False, reverse_Y=True)
        return max(score for row in scores for score in row)

    def __update_vis_scores(self,
                            grid: List[List[int]], scores: List[List[int]],
                            Y: int, X: int,
                            row_first: bool,
                            reverse_Y: bool = False,
                            reverse_X: bool = False) -> None:
        first_idx = 0 \
            if (row_first and not reverse_X) \
            or (not row_first and not reverse_Y) \
            else X - 1 if (row_first and reverse_X) else Y - 1
        for y, x in get_grid_iter(Y, X, row_first,
                                  reverse_Y=reverse_Y, reverse_X=reverse_X,
                                  new_loop_handler=self.__reset_height_idx):
            elt = grid[y][x]
            idx = x if row_first else y
            scores[y][x] *= abs(self.height_idx.get(elt, first_idx) - idx)
            for i in range(elt + 1):
                self.height_idx[i] = idx

    def __reset_height_idx(self):
        self.height_idx = {}
