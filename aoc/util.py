import time
from dataclasses import dataclass
from typing import Callable


def noop():
    pass


def identity(x):
    return x


def get_grid_iter(Y: int, X: int, row_first: bool,
                  reverse_Y: bool = False, reverse_X: bool = False,
                  new_loop_handler: Callable = noop):
    Y_transform = reversed if reverse_Y else identity
    X_transform = reversed if reverse_X else identity

    def Y_range():
        return Y_transform(range(Y))

    def X_range():
        return X_transform(range(X))
    first_range = Y_range if row_first else X_range
    second_range = X_range if row_first else Y_range
    for a in first_range():
        new_loop_handler()
        for b in second_range():
            yield (a, b) if row_first else (b, a)


def time_method(method, *args, **kwargs):
    start = time.time()
    ret = method(*args, **kwargs)
    t = time.time() - start
    return ret, t


@dataclass
class Pos:
    x: int
    y: int

    def binary_dir(self):
        """
        Returns the one of the 9 directions formed from ((-1|0|1), (-1|0|1)),
        that is closest to this vector.
        """
        dx = self.x / abs(self.x) if self.x else 0
        dy = self.y / abs(self.y) if self.y else 0

        return Pos(dx, dy)

    def __add__(self, other: 'Pos'):
        return Pos(self.x + other.x,
                   self.y + other.y)

    def __sub__(self, other: 'Pos'):
        return Pos(self.x - other.x,
                   self.y - other.y)

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Pos):
            return False
        return self.x == other.x and self.y == other.y
