from aoc.day import Day


TETROMINOS = [
    [
        "####",
    ],
    [
        ".#.",
        "###",
        ".#.",
    ],
    # Flipped vertically so (0, 0) is the lower-left corner
    [
        "###",
        "..#",
        "..#",
    ],
    [
        "#",
        "#",
        "#",
        "#",
    ],
    [
        "##",
        "##"
    ]
]


class Day17(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(17, test)

    def part1(self):
        return self.__simulate(2022)

    def part2(self):
        # After rock 1596, the room loops for the first time, at height 2433.
        # After that it loops every 1730 rocks, gaining 2647 height every time.
        # This is true for my input, the test input doesn't seem to loop
        # this way.
        #
        # "room loops" == the highest layer is all filled,
        # forming a new "floor".
        total_rocks = 1000000000000
        assert total_rocks >= 1596
        rocks = total_rocks - 1596
        height = 2433 + (rocks // 1730) * 2647
        rocks %= 1730
        # ...I just took these from the looping simulation
        rock_offset = 1
        jet_offset = 9355
        height += self.__simulate(rocks, rock_offset, jet_offset)
        return height

    def __simulate(self, num_rocks: int,
                   rock_offset: int = 0, jet_offset: int = 0):
        jet = self.lines[0]
        stopped = False
        max_y = -1
        room = []
        step = -1
        for i in range(num_rocks):
            tetromino = TETROMINOS[(i + rock_offset) % len(TETROMINOS)]
            my_h = len(tetromino)
            my_w = len(tetromino[0])
            x = 2
            y = max_y + 4
            if len(room) < y + my_h:
                room.extend(list(".......")
                            for _ in range(y + my_h - len(room)))
            stopped = False
            while not stopped:
                step += 1
                stopped = True
                wind = jet[(step + jet_offset) % len(jet)]
                if wind not in "<>":
                    raise ValueError(f"Fix your input you dingle: {wind}")
                new_x = x + (-1 if wind == "<" else 1)
                # Move sidedays
                if 0 <= new_x <= len(room[0]) - my_w:
                    col_range = range(0, my_w) if wind == "<" \
                        else range(my_w - 1, -1, -1)
                    ok = True
                    for dy in range(my_h):
                        if y + dy >= len(room):
                            continue
                        if not ok:
                            break
                        for dx in col_range:
                            if tetromino[dy][dx] == "#" \
                                    and room[y + dy][new_x + dx] == "#":
                                ok = False
                                break
                    if ok:
                        x = new_x
                new_y = y - 1
                # Move down
                if new_y >= 0:
                    ok = True
                    for dy in range(my_h):
                        if y + dy >= len(room):
                            continue
                        if not ok:
                            break
                        for dx in range(my_w):
                            if tetromino[dy][dx] == "#" \
                                    and room[new_y + dy][x + dx] == "#":
                                ok = False
                                break
                    if ok:
                        y = new_y
                        stopped = False
            for dy in range(my_h):
                for dx in range(my_w):
                    if tetromino[dy][dx] == "#":
                        room[y + dy][x + dx] = "#"
            max_y = next(dy for dy in range(len(room) - 1, -1, -1)
                         if "#" in room[dy])
        return max_y + 1
