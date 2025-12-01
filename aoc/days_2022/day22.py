from collections import defaultdict
from typing import Dict, List, Literal
from aoc.day import Day

FACE = Literal['U', 'D', 'L', 'R', 'F', 'B']
DIR = Literal['U', 'D', 'L', 'R']

DPOS: Dict[DIR, complex] = {'U': -1, 'R': 1j, 'D': 1, 'L': -1j}
CW: Dict[DIR, DIR] = {'U': 'R', 'R': 'D', 'D': 'L', 'L': 'U'}
CCW: Dict[DIR, DIR] = {v: k for k, v in CW.items()}
OPPOSITE: Dict[DIR, DIR] = {'U': 'D', 'D': 'U', 'L': 'R', 'R': 'L'}
COMPATIBLE: Dict[DIR, DIR] = {'U': 'R', 'R': 'U', 'D': 'L', 'L': 'D'}
DIR_VAL: Dict[DIR, int] = {'U': 3, 'R': 0, 'D': 1, 'L': 2}
CW_SIDE: Dict[FACE, Dict[FACE, FACE]] = {
    'F': {'L': 'U', 'U': 'R', 'R': 'D', 'D': 'L'},
    'B': {'L': 'D', 'D': 'R', 'R': 'U', 'U': 'L'},
    'L': {'B': 'U', 'U': 'F', 'F': 'D', 'D': 'B'},
    'R': {'B': 'D', 'D': 'F', 'F': 'U', 'U': 'B'},
    'U': {'L': 'B', 'B': 'R', 'R': 'F', 'F': 'L'},
    'D': {'L': 'F', 'F': 'R', 'R': 'B', 'B': 'L'},
}


class Day22(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(22, test)
        self.test = test

    def part1(self):
        grid = self.__get_grid()
        successors = {}
        max_row = int(max(i.real for i in grid))
        max_col = int(max(i.imag for i in grid))
        for dir, dpos in DPOS.items():
            succ = {}
            for row in range(1, max_row + 1):
                for col in range(1, max_col + 1):
                    pos = row + col * 1j
                    succ[pos] = self.__get_successor(pos, succ, dpos,
                                                     grid, max_row, max_col)
            successors[dir] = succ
        dir = 'R'
        pos = successors['R'][1 + 1j]
        code = self.lines[-1] + "X"
        val = 0
        for c in code:
            if c.isdigit():
                val = 10 * val + int(c)
                continue

            for _ in range(val):
                new_pos = successors[dir][pos]
                assert new_pos in grid
                if grid[new_pos] == '#':
                    break
                pos = new_pos
            val = 0
            if c == 'L':
                dir = CCW[dir]
            elif c == 'R':
                dir = CW[dir]
        return 1000 * pos.real + 4 * pos.imag + DIR_VAL[dir]

    def __get_successor(self, pos, succ, dpos, grid, max_row, max_col):
        if pos not in succ:
            new_pos = pos + dpos
            if new_pos.real < 1:
                new_pos = max_row + new_pos.imag * 1j
            elif new_pos.real > max_row:
                new_pos = 1 + new_pos.imag * 1j
            elif new_pos.imag < 1:
                new_pos = new_pos.real + max_col * 1j
            elif new_pos.imag > max_col:
                new_pos = new_pos.real + 1j
            if new_pos in grid:
                succ[pos] = new_pos
            else:
                succ[pos] = self.__get_successor(new_pos, succ, dpos,
                                                 grid, max_row, max_col)
        return succ[pos]

    def part2(self):
        grid = self.__get_grid()
        side_len = 4 if self.test else 50
        pos = min((i for i in grid if i.real == 1),
                  key=lambda i: i.imag)
        # Front, Back, Up, Down, Left, Right
        face = 'F'
        # Each side defined by its top-left corner
        face_corners: Dict[FACE, complex] = {face: pos}
        # What face is from key 1 in direction key 2
        connections: Dict[FACE, Dict[DIR, FACE]] = defaultdict(dict)
        connections[face] = {'U': 'U', 'D': 'D', 'L': 'L', 'R': 'R'}
        # Where we're facing if we go to key 1 from key 2
        new_dirs: Dict[FACE, Dict[FACE, DIR]] = defaultdict(dict)
        new_dirs[face] = {'U': 'D', 'D': 'U', 'L': 'R', 'R': 'L'}
        to_expand: List[FACE] = [face]
        """
        This is the general cube folding algorithm.
        It works as follows:
        Each side is uniquely identified by the coordinates of its
        top left corner in the input.
        1. Assume w.l.o.g. that the “first” face (the one where we start) is
        the front face. Trivially, it is connected to the left face via the
        left side of its input, to the up face via the top side of
        its input, etc. Initialize a stack and put this Front face on it.
        2. While there are faces on the stack, pop one face. For each cardinal
        direction dir from this face’s top left corner, check if the
        character side_length away in that direction is not empty.
        If so, it is another valid face. If it’s a new face, set its dir-side
        connection to be face. Then iterate clockwise for all directions,
        and put face on the stack.
        3. After step 2 completes, we should have 6 faces, each with
        4 well-defined connections to other faces.

        Then it’s just a matter of walking on the surface of these faces,
        using the connections we established above to know where we’re going.
        One thing to note is if we change between incompatible directions
        (i.e. top-left corners don’t match, e.g. from walking right to
        walking down), we need to adjust our distance dist from the corner
        as dist = side_length - 1 - dist.
        """
        while to_expand:
            face = to_expand.pop()
            corner = face_corners[face]
            for dir, dpos in DPOS.items():
                new_face = connections[face][dir]
                if new_face in face_corners:
                    continue
                new_corner = corner + side_len * dpos
                if new_corner not in grid:
                    continue
                face_corners[new_face] = new_corner
                connections[new_face][OPPOSITE[dir]] = face
                new_dirs[new_face][face] = dir
                next_face = CW_SIDE[new_face][face]
                next_dir = CW[OPPOSITE[dir]]
                while next_face != face:
                    connections[new_face][next_dir] = next_face
                    new_dirs[new_face][next_face] = OPPOSITE[next_dir]
                    next_face = CW_SIDE[new_face][next_face]
                    next_dir = CW[next_dir]
                to_expand.append(new_face)
        assert len(face_corners) == 6
        assert all(len(connections[face]) == 4 for face in face_corners)
        code = self.lines[-1] + "X"
        dir = 'R'
        face = 'F'
        pos = face_corners[face]
        val = 0
        for c in code:
            if c.isdigit():
                val = 10 * val + int(c)
                continue
            for _ in range(val):
                new_pos = pos + DPOS[dir]
                new_pos_relative = new_pos - face_corners[face]
                new_face = new_dir = sig_val = None
                if new_pos_relative.real < 0:
                    new_face = connections[face]['U']
                    sig_val = new_pos_relative.imag
                elif new_pos_relative.real >= side_len:
                    new_face = connections[face]['D']
                    sig_val = new_pos_relative.imag
                elif new_pos_relative.imag < 0:
                    new_face = connections[face]['L']
                    sig_val = new_pos_relative.real
                elif new_pos_relative.imag >= side_len:
                    new_face = connections[face]['R']
                    sig_val = new_pos_relative.real
                if new_face and sig_val is not None:
                    new_dir = new_dirs[new_face][face]
                    if new_dir not in (dir, COMPATIBLE[dir]):
                        sig_val = side_len - 1 - sig_val
                    new_pos_relative: complex = {
                        'U': side_len - 1 + sig_val * 1j,
                        'R': sig_val, 'D': sig_val * 1j,
                        'L': sig_val + (side_len - 1) * 1j}[new_dir]
                    new_pos = face_corners[new_face] + new_pos_relative
                if grid[new_pos] == '#':
                    break
                pos = new_pos
                if new_face is not None:
                    assert new_dir is not None
                    face = new_face
                    dir = new_dir
            val = 0
            if c == 'L':
                dir = CCW[dir]
            elif c == 'R':
                dir = CW[dir]
        return 1000 * pos.real + 4 * pos.imag + DIR_VAL[dir]

    def __get_grid(self):
        grid = {}
        for row, line in enumerate(self.lines[:-2], 1):
            for col, c in enumerate(line, 1):
                if c != ' ':
                    grid[row + col * 1j] = c
        return grid
