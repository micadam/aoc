import time
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
