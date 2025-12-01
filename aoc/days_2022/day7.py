from dataclasses import dataclass
from typing import Dict, List
from aoc.day import Day


class FileSystemObject(object):
    @property
    def name(self) -> str:
        return self.name_

    @property
    def path_prefix(self) -> str:
        return self.path_prefix_

    def total_size(self, size_dict=None) -> int:
        raise NotImplementedError

    def pretty_print(self, indent=""):
        raise NotImplementedError

    def __hash__(self) -> int:
        return hash((self.path_prefix, self.name))

    def __eq__(self, __o: object) -> bool:
        if not isinstance(__o, FileSystemObject):
            return False
        return self.name == __o.name and self.path_prefix == __o.path_prefix

    def __repr__(self) -> str:
        return f"{self.path_prefix}/{self.name}"


@dataclass
class File(FileSystemObject):
    name_: str
    size: int
    path_prefix_: str

    def total_size(self, _=None):
        return self.size

    def pretty_print(self, indent=""):
        print(f"{indent}- {self.name} (file, size={self.size})")


class Directory(FileSystemObject):
    def __init__(self, name: str, path_prefix: str) -> None:
        self.name_ = name
        self.children: Dict[str, FileSystemObject] = {}
        self.path_prefix_ = path_prefix

    def add_child(self, object: FileSystemObject):
        if object.name in self.children:
            assert isinstance(object, type(self.children[object.name])) \
                and (isinstance(object, Directory)
                     or object.size == self.children[object.name].size)
            return
        self.children[object.name] = object

    def total_size(self, size_dict=None):
        my_size = sum(o.total_size(size_dict) for o in self.children.values())
        if size_dict is not None:
            size_dict[self] = my_size
        return my_size

    def pretty_print(self, indent=""):
        print(f"{indent}- {self.name} (dir)")
        new_indent = indent + "  "
        for child in sorted(self.children.values(), key=lambda c: c.name):
            child.pretty_print(new_indent)


class Day7(Day):
    def __init__(self, test: bool) -> None:
        super().__init__(7, test)

    def part1(self):
        root = self.__construct_filesystem()

        size_dict = {}
        root.total_size(size_dict)

        return sum(v for v in size_dict.values() if v <= 100000)

    def part2(self):
        root = self.__construct_filesystem()

        size_dict = {}
        root_size = root.total_size(size_dict)
        missing_space = 30000000 - (70000000 - root_size)

        return min(v for v in size_dict.values() if v >= missing_space)

    def __construct_filesystem(self) -> FileSystemObject:
        assert self.lines[0] == "$ cd /"
        dir_stack: List[Directory] = []
        root = Directory("/", "")
        current_dir = root
        path_prefix = None
        for line in self.lines[1:]:
            prefix, suffix = line.rsplit(maxsplit=1)
            if prefix == "$ cd":
                if suffix == "..":
                    current_dir = dir_stack.pop()
                else:
                    dir_stack.append(current_dir)
                    # assuming ls in parent is always called first
                    assert suffix in current_dir.children
                    current_dir = current_dir.children[suffix]
            elif line == "$ ls":
                path_prefix = "/".join((*(d.name for d in dir_stack),
                                        current_dir.name))
            elif prefix == "dir":
                current_dir.add_child(Directory(suffix, path_prefix))
            elif prefix.isnumeric():
                current_dir.add_child(File(suffix, int(prefix), path_prefix))
            else:
                raise ValueError(f"Unknown command: {line}")
        return root
